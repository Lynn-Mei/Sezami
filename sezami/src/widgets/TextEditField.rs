use druid::kurbo::Circle;
use druid::piet::{Color, RenderContext, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
//use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, Env, , EventCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx, LocalizedString, PlatformError};
use druid::{AppLauncher, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens,LifeCycle,LifeCycleCtx, LocalizedString, PaintCtx,PlatformError, TextLayout, UpdateCtx, Widget, WidgetExt, WindowDesc};

pub struct TextEditField {
	text: String,
	font_size: f64,
}

impl TextEditField {
	pub fn new(text: String, font_size : f64) -> Self {
		Self { text, font_size }
	}
}

impl Widget<String> for TextEditField {
	// Paints the widget on the screen
	fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
		// Drawing a circle with white background as example
		let fill_color = Color::rgba8(0x7F, 0x7F, 0x7F, 0x7F);
		let mut text_layout = TextLayout::<String>::from_text(self.text.clone());
		text_layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(12.0));
		text_layout.set_text_color(fill_color);
		text_layout.rebuild_if_needed(ctx.text(), env);
		
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::WHITE); 
        
		text_layout.draw(ctx, (0.0, 0.0));
	}
	
	// Defines the size of the text edit field
	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &String, env: &Env) -> Size {
		Size::new(50.0, 50.0)
	}
	
	// Handles lifecycle events
	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {}
	
	// Handles events
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env){}

	// Handles updates
	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, _data: &String, env: &Env){}
	
}