use druid::kurbo::Circle;
use druid::piet::{Color, RenderContext};
use druid::widget::prelude::*;
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx};

pub struct TextEditField;

impl Widget<String> for TextEditField {
	// Paints the widget on the screen
	fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
		// Drawing a circle with white background as example
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::WHITE); 
        let circle = Circle::new((25.0, 25.0), 15.0);
        ctx.fill(circle, &Color::BLACK);
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