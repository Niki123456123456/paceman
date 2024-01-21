use std::str::FromStr;

use egui::{Button, Label, TextBuffer, TextEdit, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use reqwest::Url;
use strum::{AsStaticRef, IntoEnumIterator};

use crate::{
    client::trigger_send,
    models::{HeaderValue, Method, Request, RequestBody, RequestModel, Response},
    tabcontrol::{self, TabItem},
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    model: RequestModel,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

pub fn show_body(ui: &mut Ui, request: &mut Request) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let mut state_str = request.body.as_static();
            for body in RequestBody::iter() {
                let str = body.as_static();
                let mut response = ui.radio(state_str == str, str);
                if response.clicked() && state_str != str {
                    if let Ok(state) = RequestBody::from_str(str) {
                        state_str = str;
                        request.body = state;
                    }
                    response.mark_changed();
                }
            }
        });
    });
}

pub fn show_params(ui: &mut Ui, request: &mut Request) {
    let mut url = Url::parse("https://example.net").unwrap();
    let pairs = url.query_pairs_mut();

    ui.horizontal(|ui| {
        TableBuilder::new(ui)
            .column(Column::remainder())
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Key");
                });
                header.col(|ui| {
                    ui.heading("Value");
                });
            })
            .body(|mut body| {
                for header in request.params.iter() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            let mut key = header.key.clone();
                            TextEdit::singleline(&mut key)
                                .desired_width(ui.available_width())
                                .show(ui);
                        });
                        row.col(|ui| {
                            let mut value = header.value.clone();
                            TextEdit::singleline(&mut value)
                                .desired_width(ui.available_width())
                                .show(ui);
                        });
                    });
                }
            });
    });
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source(ui.next_auto_id())
                    .selected_text(format!("{:?}", self.model.request.method).to_uppercase())
                    .show_ui(ui, |ui| {
                        for method in Method::iter() {
                            ui.selectable_value(
                                &mut self.model.request.method,
                                method,
                                format!("{:?}", method).to_uppercase(),
                            );
                        }
                    });
                TextEdit::singleline(&mut self.model.request.url)
                    .desired_width(ui.available_width() - 40.0)
                    .show(ui);
                if Button::new("send").ui(ui).clicked() {
                    trigger_send(&self.model.request, self.model.response.clone(), &ctx);
                }
            });

            tabcontrol::show(
                ui,
                &mut self.model.request,
                vec![
                    TabItem::new("Params", show_params),
                    TabItem::new("Authorization", |ui: &mut Ui, resp: &mut Request| {}),
                    TabItem::new("Headers", |ui: &mut Ui, resp: &mut Request| {}),
                    TabItem::new("Body", show_body),
                ],
            );

            {
                let resp: &mut Option<_> = &mut self.model.response.lock().unwrap();
                if let Some(resp) = resp {
                    match resp {
                        Ok(resp) => {
                            show_resp(ui, resp);
                            tabcontrol::show(
                                ui,
                                resp,
                                vec![
                                    TabItem::new("Body", |ui: &mut Ui, resp: &mut Response| {
                                        TextEdit::multiline(&mut resp.text)
                                            .desired_width(ui.available_width())
                                            .ui(ui);
                                    }),
                                    TabItem::new("Headers", |ui: &mut Ui, resp: &mut Response| {
                                        TableBuilder::new(ui)
                                            .column(Column::remainder())
                                            .column(Column::remainder())
                                            .header(20.0, |mut header| {
                                                header.col(|ui| {
                                                    ui.heading("Key");
                                                });
                                                header.col(|ui| {
                                                    ui.heading("Value");
                                                });
                                            })
                                            .body(|mut body| {
                                                for header in resp.headers.iter() {
                                                    body.row(20.0, |mut row| {
                                                        row.col(|ui| {
                                                            let mut name = header.name.clone();
                                                            TextEdit::singleline(&mut name)
                                                                .desired_width(ui.available_width())
                                                                .show(ui);
                                                        });
                                                        row.col(|ui| match &header.value {
                                                            HeaderValue::String(str) => {
                                                                let mut value = str.clone();
                                                                TextEdit::singleline(&mut value)
                                                                    .desired_width(
                                                                        ui.available_width(),
                                                                    )
                                                                    .show(ui);
                                                            }
                                                            HeaderValue::Bytes(_) => {}
                                                        });
                                                    });
                                                }
                                            });
                                    }),
                                ],
                            );
                        }
                        Err(err) => {
                            ui.label("response");
                            ui.label(format!("err: {}", err.0));
                        }
                    }
                }
            }
        });
    }
}

fn show_resp(ui: &mut egui::Ui, resp: &Response) {
    let status = reqwest::StatusCode::from_u16(resp.status).unwrap();
    let content_length = resp
        .content_length
        .and_then(|x| Some(format!("{} B", x)))
        .unwrap_or_default();
    let duration = resp.end - resp.start;

    ui.label(format!(
        "{} {} {} ms {}",
        status.as_str(),
        status.canonical_reason().unwrap_or(""),
        duration.num_milliseconds(),
        content_length
    ));
}
