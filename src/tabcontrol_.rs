use egui::{Ui};
use egui_dock::{DockState, DockArea, Style};

pub fn show<T>(ui: &mut Ui, t :  T, menu_contents: Vec< TabItem<T>> ){ 
    let mut state = DockState::new(menu_contents);
    let mut viewer = TabViewer { t };
    DockArea::new(&mut state)
    .style(Style::from_egui(ui.style().as_ref()))
    .show(ui.ctx(), &mut viewer);
}

impl<T> TabItem<T>{
    pub fn new( name : &'static str, Fn : impl Fn(&mut Ui, &mut T) + 'static) ->  TabItem<T>{
        return TabItem{
            name, Fn: Box::new(Fn)
        };
    }
}

pub struct TabItem<T>{
    name : &'static str,
    Fn : Box<dyn Fn(&mut Ui, &mut T)>,
}

struct TabViewer<T> {
    t: T,
}

impl<T> egui_dock::TabViewer for TabViewer<T> {
    type Tab = TabItem< T>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.name.into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        (tab.Fn)(ui, &mut self.t);
    }
}