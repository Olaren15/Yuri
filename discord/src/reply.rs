use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;

pub struct Reply {
    pub message: String,
    pub link: Option<String>,
}

impl Reply {
    pub async fn send(&self, ctx: &Context, msg: &Message) {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(self.message.as_str());
                    e.color(Color::MAGENTA);
                    if let Some(link) = self.link.as_ref() {
                        e.image(link);
                    }

                    e
                });
                m
            })
            .await
            .unwrap();
    }
}
