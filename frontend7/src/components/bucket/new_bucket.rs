use datatypes::bucket::NewBucket;
use components::button::Button;
use Context;
use yew::prelude::*;
use util::input::Input;
use components::bucket::BucketModel;
use components::bucket::Msg;

use util::input::InputValidator;

impl Renderable<Context, BucketModel> for NewBucket {
    fn view(&self) -> Html<Context, BucketModel> {
        html! {
            <div class=("login-card", "flexbox-vert"),>
                <div class="flexbox-child-grow",>
                    <Input:
                        placeholder="Bucket Name",
                        input_state=&self.name,
                        on_change=|a| Msg::UpdateBucketName(a),
                        on_enter=|_| Msg::CreateBucket,
                        validator=Box::new(NewBucket::validate_name as InputValidator),
                    />
                </div>
                <Button: title="Create Bucket", onclick=|_| Msg::CreateBucket, />
            </div>
        }

    }
}