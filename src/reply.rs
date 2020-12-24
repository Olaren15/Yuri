use serenity::client::Context;
use serenity::http::AttachmentType;
use serenity::model::id::ChannelId;
use serenity::utils::Color;

use std::path::PathBuf;

pub struct Reply {
    pub message: String,
    pub attachment: Option<PathBuf>,
}

impl Reply {
    pub async fn send(&self, ctx: &Context, channel_id: &ChannelId) {
        channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(self.message.as_str());
                    e.color(Color::MAGENTA);
                    if let Some(path_buf) = self.attachment.as_ref() {
                        e.attachment(path_buf.as_path().file_name().unwrap().to_str().unwrap());
                    }

                    e
                });
                if let Some(att) = self.attachment.as_ref() {
                    m.add_file(AttachmentType::Path(att.as_path()));
                }

                m
            })
            .await
            .unwrap();
    }
}
