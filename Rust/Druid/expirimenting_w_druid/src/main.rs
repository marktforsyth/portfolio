use druid::widget::{Align, Flex, Label, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 30.0;
const BUTTON_WIDTH: f64 = 400.0;
const BUTTON_PADDING: f64 = 8.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Druid App Test");

#[derive(Clone, Data, Lens)]
struct AppState {
    x: f64,
    y: f64,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1_200.0, 600.0));

    let initial_state = AppState {
        x: 25.into(),
        y: 36.into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &Env| format!("x val is {}, y val is {}", data.x, data.y));
    let x_inc_btn = Button::new("Multiply x by 2")
        .on_click(|_ctx, data: &mut AppState, _env| data.x *= 2.0)
        .fix_width(BUTTON_WIDTH)
        .padding(BUTTON_PADDING);
    let y_inc_btn = Button::new("Subtract 3 from y")
        .on_click(|_ctx, data: &mut AppState, _env| data.y -= 3.0)
        .fix_width(BUTTON_WIDTH)
        .padding(BUTTON_PADDING);
    
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(x_inc_btn)
        .with_child(y_inc_btn);
    
    Align::centered(layout)
}