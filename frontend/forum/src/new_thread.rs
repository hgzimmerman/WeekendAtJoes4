use yew::prelude::*;
use Context;

use util::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use util::button::Button;

//use datatypes::forum::ForumData;
//use failure::Error;

//use context::networking::*;
use datatypes::thread::NewThreadData;
//use datatypes::thread::ThreadData;
//use yew::format::Json;
//use yew::services::fetch::Response;
//use yew::services::fetch::FetchTask;
//use Route;
//use forum::ForumRoute;


pub struct NewThread {
    new_thread: NewThreadData,
    callback: Callback<NewThreadData>
}


pub enum Msg {
    CreateNewThread,
    UpdatePostContent(String),
    UpdateThreadTitle(String),
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub new_thread: NewThreadData,
    pub callback: Option<Callback<NewThreadData>>
}


impl Component<Context> for NewThread {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        NewThread {
            new_thread: props.new_thread,
            callback: props.callback.expect("Didn't have a callback")
        }
    }

    fn update(&mut self, msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::CreateNewThread => {
                self.callback.emit(self.new_thread.clone());
                true
            }
            Msg::UpdateThreadTitle(title) => {
                self.new_thread.title = title;
                true
            }
            Msg::UpdatePostContent(text) => {
                self.new_thread.post_content = text;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, NewThread> for NewThread {
    fn view(&self) -> Html<Context, Self> {

        return html! {
            <div>
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="Thread Title",
                    value=&self.new_thread.title,
                    oninput=|e| Msg::UpdateThreadTitle(e.value),
//                    onkeypress=|e: KeyData| {
//                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
//                    },
                 />
                 <AuthorMarkdownToggle: callback=|text| Msg::UpdatePostContent(text), />
                 <Button: onclick=|_| Msg::CreateNewThread, />

            </div>
        };
    }
}
