use yew::prelude::*;

struct Model {
    value: i64
}

// This is like a React component
#[function_component(App)]
fn app() -> Html {
    // This is like useState in React
    let state = use_state(|| Model { 
        value: 0 
    });

    // no semicolon after the last because, 
    // otherwise onclick would return nothing, because it's a closure.
    let onclick = {
        // Shadows the original object
        let state = state.clone();
        
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
           })
        })
    };

    // This is like render in React    
    html! {
        <div>
            // {onclick} is shorthand for {onclick: onclick}
            <button {onclick}>{ "+1" }</button>
            <p>{ state.value }</p>
        </div>
    }
}

fn main() {
    // Reference to the app component
    yew::start_app::<App>();
}