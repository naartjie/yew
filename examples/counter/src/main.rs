extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

struct Model {
    value: i64,
}

enum Msg {
    Delta(i64),
    Reset,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Reset => {
                self.value = 0;
                context.console.log("resetting to 0");
            }
            Msg::Delta(delta) => {
                let old = self.value;
                self.value += delta;
                let sign = if delta > 0 {"+"} else {""};
                let msg = format!("value changed by {sign}{delta}, [old: {old}, new: {new}]",
                    delta = delta,
                    old = old,
                    new = self.value,
                    sign = sign
                );
                context.console.log(&msg);
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Reset,>{ "Reset" }</button>
                    <button onclick=|_| Msg::Delta(1),>{ "Increment" }</button>
                    <button onclick=|_| Msg::Delta(-1),>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Delta(2),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Local::now() }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
