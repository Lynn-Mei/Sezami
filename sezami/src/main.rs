use druid::widget::{Label, Button};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};

mod widgets;
use crate::widgets::TextEditField::TextEditField;

fn main() -> Result<(), PlatformError> {
	//Create Window Desc
	let main_window = WindowDesc::new(build_ui).title(LocalizedString::new("Sezami")).window_size((300.0, 150.0));
	
	//Launch App
	AppLauncher::with_window(main_window).use_simple_logger().launch("Testing Druid interface !".to_string())?;
    Ok(())
}

fn build_ui() -> impl Widget<String> {
	let text_edit_field = TextEditField::new(300.0, 300.0, "Hello World".to_string(), 12.0);
    let layout = druid::widget::Flex::column()
	.with_child(text_edit_field);
	
    layout
}
