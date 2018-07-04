(function() {var implementors = {};
implementors["fxa_email_service"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"fxa_email_service/app_errors/enum.AppErrorKind.html\" title=\"enum fxa_email_service::app_errors::AppErrorKind\">AppErrorKind</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Context&lt;<a class=\"enum\" href=\"fxa_email_service/app_errors/enum.AppErrorKind.html\" title=\"enum fxa_email_service::app_errors::AppErrorKind\">AppErrorKind</a>&gt;&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/url/1.7.0/url/parser/enum.ParseError.html\" title=\"enum url::parser::ParseError\">UrlError</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/reqwest/0.8.6/reqwest/error/struct.Error.html\" title=\"struct reqwest::error::Error\">RequestError</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/duration/struct.Duration.html\" title=\"struct fxa_email_service::duration::Duration\">Duration</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>",synthetic:false,types:[]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RedisError&gt; for <a class=\"struct\" href=\"fxa_email_service/message_data/struct.MessageDataError.html\" title=\"struct fxa_email_service::message_data::MessageDataError\">MessageDataError</a>",synthetic:false,types:["fxa_email_service::message_data::MessageDataError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;InvalidKeyLength&gt; for <a class=\"struct\" href=\"fxa_email_service/message_data/struct.MessageDataError.html\" title=\"struct fxa_email_service::message_data::MessageDataError\">MessageDataError</a>",synthetic:false,types:["fxa_email_service::message_data::MessageDataError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;SendgridError&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/str/struct.Utf8Error.html\" title=\"struct core::str::Utf8Error\">Utf8Error</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;SendRawEmailError&gt; for <a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>",synthetic:false,types:["fxa_email_service::app_errors::AppError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/queues/sqs/notification/struct.Notification.html\" title=\"struct fxa_email_service::queues::sqs::notification::Notification\">Notification</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/notification/struct.Notification.html\" title=\"struct fxa_email_service::queues::notification::Notification\">GenericNotification</a>",synthetic:false,types:["fxa_email_service::queues::notification::Notification"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"fxa_email_service/queues/sqs/notification/enum.NotificationType.html\" title=\"enum fxa_email_service::queues::sqs::notification::NotificationType\">NotificationType</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>",synthetic:false,types:["alloc::string::String"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/queues/sqs/notification/struct.Mail.html\" title=\"struct fxa_email_service::queues::sqs::notification::Mail\">Mail</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/notification/struct.Mail.html\" title=\"struct fxa_email_service::queues::notification::Mail\">GenericMail</a>",synthetic:false,types:["fxa_email_service::queues::notification::Mail"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/queues/sqs/notification/struct.Bounce.html\" title=\"struct fxa_email_service::queues::sqs::notification::Bounce\">Bounce</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/notification/struct.Bounce.html\" title=\"struct fxa_email_service::queues::notification::Bounce\">GenericBounce</a>",synthetic:false,types:["fxa_email_service::queues::notification::Bounce"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"fxa_email_service/queues/sqs/notification/enum.BounceType.html\" title=\"enum fxa_email_service::queues::sqs::notification::BounceType\">BounceType</a>&gt; for <a class=\"enum\" href=\"fxa_email_service/auth_db/enum.BounceType.html\" title=\"enum fxa_email_service::auth_db::BounceType\">AuthDbBounceType</a>",synthetic:false,types:["fxa_email_service::auth_db::BounceType"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"fxa_email_service/queues/sqs/notification/enum.BounceSubtype.html\" title=\"enum fxa_email_service::queues::sqs::notification::BounceSubtype\">BounceSubtype</a>&gt; for <a class=\"enum\" href=\"fxa_email_service/auth_db/enum.BounceSubtype.html\" title=\"enum fxa_email_service::auth_db::BounceSubtype\">AuthDbBounceSubtype</a>",synthetic:false,types:["fxa_email_service::auth_db::BounceSubtype"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/queues/sqs/notification/struct.Complaint.html\" title=\"struct fxa_email_service::queues::sqs::notification::Complaint\">Complaint</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/notification/struct.Complaint.html\" title=\"struct fxa_email_service::queues::notification::Complaint\">GenericComplaint</a>",synthetic:false,types:["fxa_email_service::queues::notification::Complaint"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"fxa_email_service/queues/sqs/notification/enum.ComplaintFeedbackType.html\" title=\"enum fxa_email_service::queues::sqs::notification::ComplaintFeedbackType\">ComplaintFeedbackType</a>&gt; for <a class=\"enum\" href=\"fxa_email_service/auth_db/enum.BounceSubtype.html\" title=\"enum fxa_email_service::auth_db::BounceSubtype\">AuthDbBounceSubtype</a>",synthetic:false,types:["fxa_email_service::auth_db::BounceSubtype"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/queues/sqs/notification/struct.Delivery.html\" title=\"struct fxa_email_service::queues::sqs::notification::Delivery\">Delivery</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/notification/struct.Delivery.html\" title=\"struct fxa_email_service::queues::notification::Delivery\">GenericDelivery</a>",synthetic:false,types:["fxa_email_service::queues::notification::Delivery"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ReceiveMessageError&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/struct.QueueError.html\" title=\"struct fxa_email_service::queues::QueueError\">QueueError</a>",synthetic:false,types:["fxa_email_service::queues::QueueError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;SendMessageError&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/struct.QueueError.html\" title=\"struct fxa_email_service::queues::QueueError\">QueueError</a>",synthetic:false,types:["fxa_email_service::queues::QueueError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;DeleteMessageError&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/struct.QueueError.html\" title=\"struct fxa_email_service::queues::QueueError\">QueueError</a>",synthetic:false,types:["fxa_email_service::queues::QueueError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.22/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">JsonError</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/struct.QueueError.html\" title=\"struct fxa_email_service::queues::QueueError\">QueueError</a>",synthetic:false,types:["fxa_email_service::queues::QueueError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fxa_email_service/app_errors/struct.AppError.html\" title=\"struct fxa_email_service::app_errors::AppError\">AppError</a>&gt; for <a class=\"struct\" href=\"fxa_email_service/queues/struct.QueueError.html\" title=\"struct fxa_email_service::queues::QueueError\">QueueError</a>",synthetic:false,types:["fxa_email_service::queues::QueueError"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()