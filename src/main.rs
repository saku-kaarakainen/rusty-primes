// imports everything from yew prelude
use yew::prelude::*;

// message that can be sent to the component
// They can be for example button clicks, input changes, etc.
enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

// Implement the Component trait for the CounterComponent struct
// In rust impl is used to implement traits
// In typescript it is used to implement interfaces
//
// for keyword is used to implement traits for structs
// in typescript it is used to implement interfaces for classes
impl Component for CounterComponent {
    // Every component must define a Message and a Properties
    type Message = Msg;

    // Properties are used to pass data to the component
    // They are like props in react
    type Properties = ();

    // create is a lifecycle method 
    // that is called when the component is created
    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // true means that the component should be re-rendered
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link()
        html! {
            <div class="container">
                <p>{ self.count }</p>
                // In rust closures are defined with |arg1, arg2, ...| { ... }
                // In typescript they are defined with (arg1, arg2, ...) => { ... }
                <button onclick=link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            </div>
        }
    }
}


fn main() {
    yew::start_app::<CounterComponent>();
}