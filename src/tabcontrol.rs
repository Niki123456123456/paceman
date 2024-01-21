use egui::Ui;

impl<T> TabItem<T> {
    pub fn new(name: &'static str, func: impl FnOnce(&mut Ui, &mut T) + 'static) -> TabItem<T> {
        return TabItem {
            name,
            func: Box::new(func),
        };
    }
}

pub struct TabItem<T> {
    name: &'static str,
    func: Box<dyn FnOnce(&mut Ui, &mut T)>,
}

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct TabState(Option<String>);

pub fn show<T>(ui: &mut Ui, t: &mut T, tabs: Vec<TabItem<T>>) {
    if tabs.len() > 0 {
        ui.vertical(|ui| {
            let id = ui.next_auto_id();
            let mut state: TabState = ui
                .ctx()
                .data_mut(|d| d.get_persisted(id))
                .unwrap_or_default();
            if state.0.is_none() {
                state = TabState(Some(tabs.first().unwrap().name.to_string()));
            }
            ui.horizontal(|ui| {
                for tab in tabs.iter() {
                    ui.selectable_value(&mut state.0, Some(tab.name.to_string()), tab.name);
                }
            });
           
            for tab in tabs.into_iter() {
                if Some(tab.name.to_string()) == state.0 {
                    (tab.func)(ui, t);
                }
            }
            ui.ctx().data_mut(|d| d.insert_persisted(id, state));
        });
    }
}
