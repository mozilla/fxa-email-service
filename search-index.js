var N = null;var searchIndex = {};
searchIndex["fxa_email_queues"]={"doc":"The queue-processing loop for fxa_email_service.","items":[],"paths":[]};
searchIndex["fxa_email_send"]={"doc":"The main process for fxa-email-service. Starts a Rocket server that exposes one endpoint: `POST /send`","items":[],"paths":[]};
searchIndex["fxa_email_service"]={"doc":"These are the developer docs for the Firefox Accounts email-sending service. For higher-level documentation, see the [readme].","items":[[0,"app_errors","fxa_email_service","Error definitions.",N,N],[3,"AppError","fxa_email_service::app_errors","The main error type returned by this service.",N,N],[4,"AppErrorKind","","The specific kind of error that can occur.",N,N],[13,"BadRequest","","400 Bad Request",0,N],[13,"NotFound","","404 Not Found",0,N],[13,"MethodNotAllowed","","405 Method Not Allowed",0,N],[13,"UnprocessableEntity","","422 Unprocessable Entity",0,N],[13,"TooManyRequests","","429 Too Many Requests,",0,N],[13,"InternalServerError","","500 Internal Server Error",0,N],[13,"RocketError","","",0,N],[13,"InvalidEmailParams","","An error for invalid email params in the /send handler.",0,N],[13,"MissingEmailParams","","An error for missing email params in the /send handler.",0,N],[13,"InvalidProvider","","An error for invalid provider names.",0,N],[13,"ProviderError","","An error for when we get an error from a provider.",0,N],[12,"name","fxa_email_service::app_errors::AppErrorKind","",0,N],[12,"description","","",0,N],[13,"EmailParsingError","fxa_email_service::app_errors","An error for when we have trouble parsing the email message.",0,N],[13,"BounceComplaintError","","An error for when a bounce violation happens.",0,N],[12,"address","fxa_email_service::app_errors::AppErrorKind","",0,N],[12,"bounce","","",0,N],[13,"BounceSoftError","fxa_email_service::app_errors","",0,N],[12,"address","fxa_email_service::app_errors::AppErrorKind","",0,N],[12,"bounce","","",0,N],[13,"BounceHardError","fxa_email_service::app_errors","",0,N],[12,"address","fxa_email_service::app_errors::AppErrorKind","",0,N],[12,"bounce","","",0,N],[13,"DbError","fxa_email_service::app_errors","An error for when an error happens on a request to the db.",0,N],[13,"NotImplemented","","An error for when we try to access functionality that is not implemented.",0,N],[5,"bad_request","","",N,[[],["appresult"]]],[5,"not_found","","",N,[[],["appresult"]]],[5,"method_not_allowed","","",N,[[],["appresult"]]],[5,"unprocessable_entity","","",N,[[],["appresult"]]],[5,"too_many_requests","","",N,[[],["appresult"]]],[5,"internal_server_error","","",N,[[],["appresult"]]],[6,"AppResult","","",N,N],[7,"static_rocket_catch_info_for_bad_request","","Rocket code generated static catch information structure.",N,N],[7,"static_rocket_catch_info_for_not_found","","Rocket code generated static catch information structure.",N,N],[7,"static_rocket_catch_info_for_method_not_allowed","","Rocket code generated static catch information structure.",N,N],[7,"static_rocket_catch_info_for_unprocessable_entity","","Rocket code generated static catch information structure.",N,N],[7,"static_rocket_catch_info_for_too_many_requests","","Rocket code generated static catch information structure.",N,N],[7,"static_rocket_catch_info_for_internal_server_error","","Rocket code generated static catch information structure.",N,N],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"json","","",1,[[["self"]],["jsonvalue"]]],[11,"kind","","",1,[[["self"]],["apperrorkind"]]],[11,"cause","","",1,[[["self"]],["option",["fail"]]]],[11,"backtrace","","",1,[[["self"]],["option",["backtrace"]]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"clone","","",0,[[["self"]],["apperrorkind"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"cause","","",0,[[["self"]],["option",["fail"]]]],[11,"backtrace","","",0,[[["self"]],["option",["backtrace"]]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"eq","","",0,[[["self"],["apperrorkind"]],["bool"]]],[11,"ne","","",0,[[["self"],["apperrorkind"]],["bool"]]],[11,"http_status","","Return a rocket response Status to be rendered for an error",0,[[["self"]],["status"]]],[11,"errno","","",0,[[["self"]],["option",["i32"]]]],[11,"additional_fields","","",0,[[["self"]],["map",["string","value"]]]],[11,"from","","",1,[[["apperrorkind"]],["apperror"]]],[11,"from","","",1,[[["context",["apperrorkind"]]],["apperror"]]],[11,"respond_to","","",1,[[["self"],["request"]],["result"]]],[0,"auth_db","fxa_email_service","Strongly-typed wrapper for a subset of [`fxa-auth-db-mysql`][authdb].",N,N],[3,"BounceRecord","fxa_email_service::auth_db","",N,N],[12,"address","","",2,N],[12,"bounce_type","","",2,N],[12,"bounce_subtype","","",2,N],[12,"created_at","","",2,N],[3,"DbClient","","",N,N],[4,"BounceType","","",N,N],[13,"Hard","","",3,N],[13,"Soft","","",3,N],[13,"Complaint","","",3,N],[4,"BounceSubtype","","",N,N],[13,"Unmapped","","",4,N],[13,"Undetermined","","",4,N],[13,"General","","",4,N],[13,"NoEmail","","",4,N],[13,"Suppressed","","",4,N],[13,"MailboxFull","","",4,N],[13,"MessageTooLarge","","",4,N],[13,"ContentRejected","","",4,N],[13,"AttachmentRejected","","",4,N],[13,"Abuse","","",4,N],[13,"AuthFailure","","",4,N],[13,"Fraud","","",4,N],[13,"NotSpam","","",4,N],[13,"Other","","",4,N],[13,"Virus","","",4,N],[8,"Db","","",N,N],[10,"get_bounces","","",5,[[["self"],["str"]],["appresult",["vec"]]]],[11,"create_bounce","","",5,[[["self"],["str"],["bouncetype"],["bouncesubtype"]],["appresult"]]],[11,"clone","","",3,[[["self"]],["bouncetype"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"hash","","",3,N],[11,"eq","","",3,[[["self"],["bouncetype"]],["bool"]]],[11,"deserialize","","",3,[[["d"]],["result"]]],[11,"serialize","","",3,[[["self"],["s"]],["result"]]],[11,"clone","","",4,[[["self"]],["bouncesubtype"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"eq","","",4,[[["self"],["bouncesubtype"]],["bool"]]],[11,"deserialize","","",4,[[["d"]],["result"]]],[11,"serialize","","",4,[[["self"],["s"]],["result"]]],[11,"clone","","",2,[[["self"]],["bouncerecord"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"eq","","",2,[[["self"],["bouncerecord"]],["bool"]]],[11,"ne","","",2,[[["self"],["bouncerecord"]],["bool"]]],[11,"from","fxa_email_service::app_errors","",1,[[["urlerror"]],["apperror"]]],[11,"from","","",1,[[["requesterror"]],["apperror"]]],[11,"fmt","fxa_email_service::auth_db","",6,[[["self"],["formatter"]],["result"]]],[11,"new","","",6,[[["settings"]],["dbclient"]]],[11,"get_bounces","","",6,[[["self"],["str"]],["appresult",["vec"]]]],[11,"create_bounce","","",6,[[["self"],["str"],["bouncetype"],["bouncesubtype"]],["appresult"]]],[0,"bounces","fxa_email_service","Bounce and complaint handling.",N,N],[3,"Bounces","fxa_email_service::bounces","Bounce/complaint registry.",N,N],[11,"fmt","","",7,[[["self"],["formatter"]],["result"]]],[11,"new","","Instantiate the registry.",7,[[["settings"],["d"]],["bounces"]]],[11,"check","","Check an email address against bounce/complaint records from the registry.",7,[[["self"],["str"]],["appresult"]]],[11,"record_bounce","","Record a hard or soft bounce against an email address.",7,[[["self"],["str"],["bouncetype"],["bouncesubtype"]],["appresult"]]],[11,"record_complaint","","Record a complaint against an email address.",7,[[["self"],["str"],["option",["complaintfeedbacktype"]]],["appresult"]]],[0,"duration","fxa_email_service","Maps duration strings to millisecond values.",N,N],[3,"DurationError","fxa_email_service::duration","The error type returned by `Duration::try_from`.",N,N],[12,"value","","",8,N],[3,"Duration","","A duration type represented in milliseconds, for compatibility with the rest of the FxA ecosystem.",N,N],[12,"0","","",9,N],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"description","","",8,[[["self"]],["str"]]],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"clone","","",9,[[["self"]],["duration"]]],[11,"fmt","","",9,[[["self"],["formatter"]],["result"]]],[11,"default","","",9,[[],["duration"]]],[11,"eq","","",9,[[["self"],["duration"]],["bool"]]],[11,"ne","","",9,[[["self"],["duration"]],["bool"]]],[11,"deserialize","","Validate and deserialize a duration from a string of the format `\"{number} {period}\"`, e.g. `\"1 hour\"` or `\"10 minutes\"`.",9,[[["d"]],["result"]]],[11,"try_from","","",9,[[["str"]],["result",["duration","durationerror"]]]],[0,"email_address","fxa_email_service","Email address type.",N,N],[3,"EmailAddress","fxa_email_service::email_address","",N,N],[12,"0","","",10,N],[11,"clone","","",10,[[["self"]],["emailaddress"]]],[11,"fmt","","",10,[[["self"],["formatter"]],["result"]]],[11,"default","","",10,[[],["emailaddress"]]],[11,"eq","","",10,[[["self"],["emailaddress"]],["bool"]]],[11,"ne","","",10,[[["self"],["emailaddress"]],["bool"]]],[11,"deserialize","","",10,[[["d"]],["result"]]],[0,"healthcheck","fxa_email_service","Route handlers for our heathcheck endpoints: for the `GET /__version__` endpoint, for the `GET /__lbheartbeat__` endpoint and for the `GET /__heartbeat__` endpoint,",N,N],[5,"rocket_route_fn_version","fxa_email_service::healthcheck","",N,[[["request"],["data"]],["outcome"]]],[5,"version","","",N,[[],["json",["jsonvalue"]]]],[5,"rocket_route_fn_lbheartbeat","","",N,[[["request"],["data"]],["outcome"]]],[5,"lbheartbeat","","",N,[[],["json",["jsonvalue"]]]],[5,"rocket_route_fn_heartbeat","","",N,[[["request"],["data"]],["outcome"]]],[5,"heartbeat","","",N,[[["state",["settings"]]],["appresult",["json"]]]],[7,"static_rocket_route_info_for_version","","Rocket code generated static route information structure.",N,N],[7,"static_rocket_route_info_for_lbheartbeat","","Rocket code generated static route information structure.",N,N],[7,"static_rocket_route_info_for_heartbeat","","Rocket code generated static route information structure.",N,N],[0,"logging","fxa_email_service","Mozlog-compatible logging.",N,N],[3,"MozlogLogger","fxa_email_service::logging","Mozlog-compatible logger type.",N,N],[11,"new","","Construct a logger.",11,[[["settings"]],["result",["mozloglogger","error"]]]],[11,"with_request","","Log a rocket request.",11,[[["request"]],["result",["mozloglogger","error"]]]],[11,"deref","","",11,N],[0,"message_data","fxa_email_service","Temporary storage for message metadata.",N,N],[3,"MessageData","fxa_email_service::message_data","Message data store.",N,N],[3,"MessageDataError","","The error type returned by `MessageData` methods.",N,N],[11,"fmt","","",12,[[["self"],["formatter"]],["result"]]],[11,"new","","Instantiate a storage client.",12,[[["settings"]],["messagedata"]]],[11,"consume","","Consume (read and delete) message metadata.",12,[[["self"],["str"]],["result",["string","messagedataerror"]]]],[11,"set","","Store message metadata.",12,[[["self"],["str"],["str"]],["result",["messagedataerror"]]]],[11,"fmt","","",13,[[["self"],["formatter"]],["result"]]],[11,"description","","",13,[[["self"]],["str"]]],[11,"fmt","","",13,[[["self"],["formatter"]],["result"]]],[11,"from","","",13,[[["rediserror"]],["messagedataerror"]]],[11,"from","","",13,[[["invalidkeylength"]],["messagedataerror"]]],[0,"providers","fxa_email_service","Generic abstraction of specific email providers.",N,N],[3,"Providers","fxa_email_service::providers","Generic provider wrapper.",N,N],[11,"from","fxa_email_service::app_errors","",1,[[["sendgriderror"]],["apperror"]]],[11,"from","","",1,[[["utf8error"]],["apperror"]]],[11,"from","","",1,[[["string"]],["self"]]],[11,"from","","",1,[[["sendrawemailerror"]],["apperror"]]],[11,"from","","",1,[[["smtperror"]],["apperror"]]],[11,"from","","",1,[[["emailerror"]],["apperror"]]],[11,"from","","",1,[[["socketlabserror"]],["apperror"]]],[6,"Headers","fxa_email_service::providers","Email headers.",N,N],[11,"new","","Instantiate the provider clients.",14,[[["settings"]],["providers"]]],[11,"send","","Send an email via an underlying provider.",14,N],[0,"queues","fxa_email_service","Queue-processing abstractions.",N,N],[3,"Queues","fxa_email_service::queues","Top-level queue wrapper.",N,N],[3,"Message","","Generic message type.",N,N],[12,"id","","",15,N],[12,"notification","","",15,N],[3,"QueueError","","The error type returned by queue methods.",N,N],[3,"QueueIds","","Queue \"ids\" (which is really just a generic name for SQS queue URLs).",N,N],[12,"bounce","","",16,N],[12,"complaint","","",16,N],[12,"delivery","","",16,N],[12,"notification","","",16,N],[0,"notification","","Generic queue notification types.",N,N],[3,"Notification","fxa_email_service::queues::notification","The root notification type.",N,N],[12,"notification_type","","",17,N],[12,"mail","","",17,N],[12,"metadata","","",17,N],[12,"bounce","","",17,N],[12,"complaint","","",17,N],[12,"delivery","","",17,N],[3,"Mail","","",N,N],[12,"timestamp","","",18,N],[12,"message_id","","",18,N],[12,"source","","",18,N],[12,"destination","","",18,N],[12,"headers","","",18,N],[3,"Bounce","","",N,N],[12,"bounce_type","","",19,N],[12,"bounce_subtype","","",19,N],[12,"bounced_recipients","","",19,N],[12,"timestamp","","",19,N],[3,"Complaint","","",N,N],[12,"complained_recipients","","",20,N],[12,"complaint_feedback_type","","",20,N],[12,"timestamp","","",20,N],[3,"Delivery","","",N,N],[12,"timestamp","","",21,N],[12,"recipients","","",21,N],[11,"fmt","","",17,[[["self"],["formatter"]],["result"]]],[11,"default","","",17,[[],["notification"]]],[11,"fmt","","",18,[[["self"],["formatter"]],["result"]]],[11,"default","","",18,[[],["mail"]]],[11,"fmt","","",19,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",20,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",21,[[["self"],["formatter"]],["result"]]],[0,"sqs","fxa_email_service::queues","Concrete trait implementations for AWS SQS queues.",N,N],[3,"Queue","fxa_email_service::queues::sqs","An AWS SQS queue type.",N,N],[0,"notification","","SQS queue notification types.",N,N],[3,"Notification","fxa_email_service::queues::sqs::notification","The root SQS queue notification type.",N,N],[12,"notification_type","","",22,N],[12,"mail","","",22,N],[12,"bounce","","",22,N],[12,"complaint","","",22,N],[12,"delivery","","",22,N],[3,"Mail","","",N,N],[3,"Header","","",N,N],[3,"Bounce","","",N,N],[12,"bounce_type","","",23,N],[12,"bounce_subtype","","",23,N],[12,"bounced_recipients","","",23,N],[12,"timestamp","","",23,N],[12,"feedback_id","","",23,N],[12,"remote_mta_ip","","",23,N],[12,"reporting_mta","","",23,N],[3,"BouncedRecipient","","",N,N],[12,"email_address","","",24,N],[12,"action","","",24,N],[12,"status","","",24,N],[12,"diagnostic_code","","",24,N],[3,"Complaint","","",N,N],[12,"complained_recipients","","",25,N],[12,"timestamp","","",25,N],[12,"feedback_id","","",25,N],[12,"user_agent","","",25,N],[12,"complaint_feedback_type","","",25,N],[12,"arrival_date","","",25,N],[3,"ComplainedRecipient","","",N,N],[12,"email_address","","",26,N],[3,"Delivery","","",N,N],[12,"timestamp","","",27,N],[12,"processing_time_millis","","",27,N],[12,"recipients","","",27,N],[12,"smtp_response","","",27,N],[12,"remote_mta_ip","","",27,N],[12,"reporting_mta","","",27,N],[4,"NotificationType","","",N,N],[13,"Bounce","","",28,N],[13,"Complaint","","",28,N],[13,"Delivery","","",28,N],[13,"Null","","",28,N],[4,"HeaderValue","","",N,N],[13,"Single","","",29,N],[13,"Multiple","","",29,N],[4,"BounceType","","",N,N],[13,"Undetermined","","",30,N],[13,"Permanent","","",30,N],[13,"Transient","","",30,N],[4,"BounceSubtype","","",N,N],[13,"Undetermined","","",31,N],[13,"General","","",31,N],[13,"NoEmail","","",31,N],[13,"Suppressed","","",31,N],[13,"MailboxFull","","",31,N],[13,"MessageTooLarge","","",31,N],[13,"ContentRejected","","",31,N],[13,"AttachmentRejected","","",31,N],[4,"ComplaintFeedbackType","","",N,N],[13,"Abuse","","",32,N],[13,"AuthFailure","","",32,N],[13,"Fraud","","",32,N],[13,"NotSpam","","",32,N],[13,"Other","","",32,N],[13,"Virus","","",32,N],[11,"fmt","","",22,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues::notification","",17,[[["notification"]],["genericnotification"]]],[11,"clone","fxa_email_service::queues::sqs::notification","",28,[[["self"]],["notificationtype"]]],[11,"fmt","","",28,[[["self"],["formatter"]],["result"]]],[11,"eq","","",28,[[["self"],["notificationtype"]],["bool"]]],[11,"fmt","","",28,[[["self"],["formatter"]],["result"]]],[11,"default","","",28,[[],["notificationtype"]]],[11,"deserialize","","",28,[[["d"]],["result"]]],[11,"serialize","","",28,[[["self"],["s"]],["result"]]],[11,"fmt","","",33,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues::notification","",18,[[["mail"]],["genericmail"]]],[11,"fmt","fxa_email_service::queues::sqs::notification","",34,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",29,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",23,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues::notification","",19,[[["bounce"]],["genericbounce"]]],[11,"clone","fxa_email_service::queues::sqs::notification","",30,[[["self"]],["bouncetype"]]],[11,"fmt","","",30,[[["self"],["formatter"]],["result"]]],[11,"eq","","",30,[[["self"],["bouncetype"]],["bool"]]],[11,"from","fxa_email_service::auth_db","",3,[[["bouncetype"]],["authdbbouncetype"]]],[11,"deserialize","fxa_email_service::queues::sqs::notification","",30,[[["d"]],["result"]]],[11,"clone","","",31,[[["self"]],["bouncesubtype"]]],[11,"fmt","","",31,[[["self"],["formatter"]],["result"]]],[11,"eq","","",31,[[["self"],["bouncesubtype"]],["bool"]]],[11,"from","fxa_email_service::auth_db","",4,[[["bouncesubtype"]],["authdbbouncesubtype"]]],[11,"deserialize","fxa_email_service::queues::sqs::notification","",31,[[["d"]],["result"]]],[11,"fmt","","",24,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",25,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues::notification","",20,[[["complaint"]],["genericcomplaint"]]],[11,"fmt","fxa_email_service::queues::sqs::notification","",26,[[["self"],["formatter"]],["result"]]],[11,"clone","","",32,[[["self"]],["complaintfeedbacktype"]]],[11,"fmt","","",32,[[["self"],["formatter"]],["result"]]],[11,"eq","","",32,[[["self"],["complaintfeedbacktype"]],["bool"]]],[11,"from","fxa_email_service::auth_db","",4,[[["complaintfeedbacktype"]],["authdbbouncesubtype"]]],[11,"deserialize","fxa_email_service::queues::sqs::notification","",32,[[["d"]],["result"]]],[11,"serialize","","",32,[[["self"],["s"]],["result"]]],[11,"fmt","","",27,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues::notification","",21,[[["delivery"]],["genericdelivery"]]],[11,"new","fxa_email_service::queues::sqs","",35,[[["string"],["settings"]],["queue"]]],[11,"receive","","",35,[[["self"]],["box",["future"]]]],[11,"delete","","",35,[[["self"],["message"]],["box",["future"]]]],[11,"send","","",35,[[["self"],["notification"]],["box",["future"]]]],[11,"fmt","","",35,[[["self"],["formatter"]],["result"]]],[11,"from","fxa_email_service::queues","",36,[[["receivemessageerror"]],["queueerror"]]],[11,"from","","",36,[[["sendmessageerror"]],["queueerror"]]],[11,"from","","",36,[[["deletemessageerror"]],["queueerror"]]],[11,"from","","",36,[[["jsonerror"]],["queueerror"]]],[8,"Incoming","","An incoming bounce/complaint queue.",N,N],[10,"receive","","",37,[[["self"]],["box",["future"]]]],[10,"delete","","",37,[[["self"],["message"]],["box",["future"]]]],[8,"Outgoing","","An outgoing notification queue.",N,N],[10,"send","","",38,[[["self"],["notification"]],["box",["future"]]]],[8,"Factory","","A queue factory.",N,N],[10,"new","","",39,[[["string"],["settings"]],["self"]]],[11,"fmt","","",40,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",15,[[["self"],["formatter"]],["result"]]],[11,"default","","",15,[[],["message"]]],[11,"fmt","","",36,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",16,[[["self"],["formatter"]],["result"]]],[11,"new","","Instantiate the queue clients.",40,[[["queueids"],["settings"]],["queues"]]],[11,"process","","Poll all queues and handle any notifications.",40,[[["self"]],["box",["future"]]]],[11,"new","","",36,[[["string"]],["queueerror"]]],[11,"description","","",36,[[["self"]],["str"]]],[11,"fmt","","",36,[[["self"],["formatter"]],["result"]]],[11,"from","","",36,[[["apperror"]],["queueerror"]]],[0,"send","fxa_email_service","Route handler for the `POST /send` endpoint.",N,N],[3,"Body","fxa_email_service::send","",N,N],[3,"Email","","",N,N],[5,"validate","","",N,[[["email"]],["bool"]]],[5,"rocket_route_fn_handler","","",N,[[["request"],["data"]],["outcome"]]],[5,"handler","","",N,[[["appresult",["email"]],["state",["bounces"]],["state",["messagedata"]],["state",["providers"]]],["appresult",["json"]]]],[7,"static_rocket_route_info_for_handler","","Rocket code generated static route information structure.",N,N],[17,"_IMPL_DESERIALIZE_FOR_Body","","",N,N],[17,"_IMPL_DESERIALIZE_FOR_Email","","",N,N],[11,"fmt","","",41,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",42,[[["self"],["formatter"]],["result"]]],[11,"from_data","","",42,[[["request"],["data"]],["outcome"]]],[0,"serialize","fxa_email_service","Serialization functions for use with serde's `serialize_with` attribute.",N,N],[5,"hidden_or_not_set","fxa_email_service::serialize","Serialize an `Option` containing sensitive data to either of the strings `\"[hidden]\"` or `\"[not set]\"`.",N,[[["option"],["s"]],["result"]]],[5,"hidden","","Serialize any sensitive data to `\"[hidden]\"`.",N,[[["t"],["s"]],["result"]]],[0,"settings","fxa_email_service","Application settings.",N,N],[3,"AwsAccess","fxa_email_service::settings","AWS access key type.",N,N],[12,"0","","",43,N],[3,"AwsRegion","","AWS region type.",N,N],[12,"0","","",44,N],[3,"AwsSecret","","AWS secret key type.",N,N],[12,"0","","",45,N],[3,"BaseUri","","Base URI type.",N,N],[12,"0","","",46,N],[3,"DefaultProvider","","Default email provider.",N,N],[12,"0","","",47,N],[3,"Env","","Env type.",N,N],[12,"0","","",48,N],[3,"Host","","Host name or IP address type.",N,N],[12,"0","","",49,N],[3,"LoggingLevel","","Logging level type.",N,N],[12,"0","","",50,N],[3,"LoggingFormat","","Logging format type.",N,N],[12,"0","","",51,N],[3,"SenderName","","Sender name type.",N,N],[12,"0","","",52,N],[3,"SendgridApiKey","","Sendgrid API key type.",N,N],[12,"0","","",53,N],[3,"SqsUrl","","AWS SQS queue URL type.",N,N],[12,"0","","",54,N],[3,"AuthDb","","Settings related to `fxa-auth-db-mysql`, which is used to store bounce, complaint and delivery notifications.",N,N],[12,"baseuri","","The base URI for the `fxa-auth-db-mysql` instance.",55,N],[3,"Aws","","Settings for AWS.",N,N],[12,"keys","","Controls the access and secret keys for connecting to AWS.",56,N],[12,"region","","The AWS region for SES and SQS.",56,N],[12,"sqsurls","","URLs for SQS queues.",56,N],[3,"AwsKeys","","AWS keys. These are sensitive data and will not be logged.",N,N],[12,"access","","The AWS access key.",57,N],[12,"secret","","The AWS secret key.",57,N],[3,"BounceLimit","","A definition object for a bounce/complaint limit.",N,N],[12,"period","","The time period within which to limit bounces/complaints.",58,N],[12,"limit","","The maximum number of bounces/complaints to permit within the specified time period.",58,N],[3,"BounceLimits","","Controls the thresholds and behaviour for bounce and complaint reports.",N,N],[12,"enabled","","Controls whether to enable bounce limits. If set to `false`, bounce and complaint records in the database are ignored.",59,N],[12,"complaint","","Limits for complaints/spam reports.",59,N],[12,"hard","","Limits for hard (permanent) bounces.",59,N],[12,"soft","","Limits for soft (transient) bounces.",59,N],[3,"Log","","Settings for logging.",N,N],[12,"level","","The logging level.",60,N],[12,"format","","The logging format.",60,N],[3,"Provider","","Email provider settings.",N,N],[12,"default","","The default email provider to use, can be `\"ses\"`, `\"sendgrid\"`, `\"socketlabs\"`, `\"smtp\"` or `\"mock\"`. Note that this setting can be overridden on a per-request basis unless `forcedefault` is `true`.",61,N],[12,"forcedefault","","Flag indicating whether the default provider should be enforced in preference to the per-request `provider` param.",61,N],[3,"Redis","","Settings for Redis.",N,N],[12,"host","","The host name or IP address.",62,N],[12,"port","","TCP port number.",62,N],[3,"Sender","","Controls the name and email address that are used for the `From` and `Sender` email headers.",N,N],[12,"address","","The email address.",63,N],[12,"name","","The name (may contain spaces).",63,N],[3,"Sendgrid","","Settings for Sendgrid.",N,N],[12,"key","","The API key. This is sensitive data and will not be logged.",64,N],[3,"Smtp","","Settings for SMTP custom provider.",N,N],[12,"host","","SMTP host IP address.",65,N],[12,"port","","SMTP host port.",65,N],[12,"credentials","","Optional SMTP credentials.",65,N],[3,"SmtpCredentials","","Settings for SMTP custom provider optional credentials.",N,N],[12,"user","","SMTP user.",66,N],[12,"password","","SMTP password.",66,N],[3,"SocketLabs","","Settings for SocketLabs.",N,N],[12,"serverid","","The server ID. This is sensitive data and will not be logged.",67,N],[12,"key","","The API key. This is sensitive data and will not be logged.",67,N],[3,"SqsUrls","","URLs for SQS queues.",N,N],[12,"bounce","","The incoming bounce queue URL.",68,N],[12,"complaint","","The incoming complaint queue URL.",68,N],[12,"delivery","","The incoming delivery queue URL.",68,N],[12,"notification","","The outgoing notification queue URL, used to forward notifications for additional processing by callers.",68,N],[3,"Settings","","The root settings object.",N,N],[12,"authdb","","Settings related to `fxa-auth-db-mysql`, which is used to store bounce, complaint and delivery notifications.",69,N],[12,"aws","","Settings for AWS, including region, access keys and URLs for SQS queues.",69,N],[12,"bouncelimits","","Controls the thresholds and behaviour for bounce and complaint reports. If bounce limits are enabled, emails sent to offending addresses will fail with a `429` error.",69,N],[12,"env","","The env sets which environment we are in. It defaults to `dev` if not set.",69,N],[12,"hmackey","","The HMAC key to use internally for hashing message ids. This is sensitive data and will not be logged.",69,N],[12,"host","","",69,N],[12,"log","","The logging settings, about level and formatting.",69,N],[12,"port","","The port this application is listening to.",69,N],[12,"provider","","Settings controlling the default email provider.",69,N],[12,"redis","","Settings for Redis, which is used to store metadata associated with a message.",69,N],[12,"secretkey","","Setting for a secret key used by Rocket.",69,N],[12,"sender","","Controls the name and email address that are used for the `From` and `Sender` email headers.",69,N],[12,"sendgrid","","Settings for Sendgrid.",69,N],[12,"smtp","","Settings for SMTP custom provider.",69,N],[12,"socketlabs","","Settings for SocketLabs.",69,N],[11,"clone","","",43,[[["self"]],["awsaccess"]]],[11,"fmt","","",43,[[["self"],["formatter"]],["result"]]],[11,"default","","",43,[[],["awsaccess"]]],[11,"eq","","",43,[[["self"],["awsaccess"]],["bool"]]],[11,"ne","","",43,[[["self"],["awsaccess"]],["bool"]]],[11,"fmt","","",43,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",43,[[["d"]],["result"]]],[11,"clone","","",44,[[["self"]],["awsregion"]]],[11,"fmt","","",44,[[["self"],["formatter"]],["result"]]],[11,"default","","",44,[[],["awsregion"]]],[11,"eq","","",44,[[["self"],["awsregion"]],["bool"]]],[11,"ne","","",44,[[["self"],["awsregion"]],["bool"]]],[11,"fmt","","",44,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",44,[[["d"]],["result"]]],[11,"clone","","",45,[[["self"]],["awssecret"]]],[11,"fmt","","",45,[[["self"],["formatter"]],["result"]]],[11,"default","","",45,[[],["awssecret"]]],[11,"eq","","",45,[[["self"],["awssecret"]],["bool"]]],[11,"ne","","",45,[[["self"],["awssecret"]],["bool"]]],[11,"fmt","","",45,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",45,[[["d"]],["result"]]],[11,"clone","","",46,[[["self"]],["baseuri"]]],[11,"fmt","","",46,[[["self"],["formatter"]],["result"]]],[11,"default","","",46,[[],["baseuri"]]],[11,"eq","","",46,[[["self"],["baseuri"]],["bool"]]],[11,"ne","","",46,[[["self"],["baseuri"]],["bool"]]],[11,"fmt","","",46,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",46,[[["d"]],["result"]]],[11,"clone","","",47,[[["self"]],["defaultprovider"]]],[11,"fmt","","",47,[[["self"],["formatter"]],["result"]]],[11,"default","","",47,[[],["defaultprovider"]]],[11,"eq","","",47,[[["self"],["defaultprovider"]],["bool"]]],[11,"ne","","",47,[[["self"],["defaultprovider"]],["bool"]]],[11,"fmt","","",47,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",47,[[["d"]],["result"]]],[11,"clone","","",48,[[["self"]],["env"]]],[11,"fmt","","",48,[[["self"],["formatter"]],["result"]]],[11,"default","","",48,[[],["env"]]],[11,"eq","","",48,[[["self"],["env"]],["bool"]]],[11,"ne","","",48,[[["self"],["env"]],["bool"]]],[11,"fmt","","",48,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",48,[[["d"]],["result"]]],[11,"clone","","",49,[[["self"]],["host"]]],[11,"fmt","","",49,[[["self"],["formatter"]],["result"]]],[11,"default","","",49,[[],["host"]]],[11,"eq","","",49,[[["self"],["host"]],["bool"]]],[11,"ne","","",49,[[["self"],["host"]],["bool"]]],[11,"fmt","","",49,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",49,[[["d"]],["result"]]],[11,"clone","","",50,[[["self"]],["logginglevel"]]],[11,"fmt","","",50,[[["self"],["formatter"]],["result"]]],[11,"default","","",50,[[],["logginglevel"]]],[11,"eq","","",50,[[["self"],["logginglevel"]],["bool"]]],[11,"ne","","",50,[[["self"],["logginglevel"]],["bool"]]],[11,"fmt","","",50,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",50,[[["d"]],["result"]]],[11,"clone","","",51,[[["self"]],["loggingformat"]]],[11,"fmt","","",51,[[["self"],["formatter"]],["result"]]],[11,"default","","",51,[[],["loggingformat"]]],[11,"eq","","",51,[[["self"],["loggingformat"]],["bool"]]],[11,"ne","","",51,[[["self"],["loggingformat"]],["bool"]]],[11,"fmt","","",51,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",51,[[["d"]],["result"]]],[11,"clone","","",52,[[["self"]],["sendername"]]],[11,"fmt","","",52,[[["self"],["formatter"]],["result"]]],[11,"default","","",52,[[],["sendername"]]],[11,"eq","","",52,[[["self"],["sendername"]],["bool"]]],[11,"ne","","",52,[[["self"],["sendername"]],["bool"]]],[11,"fmt","","",52,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",52,[[["d"]],["result"]]],[11,"clone","","",53,[[["self"]],["sendgridapikey"]]],[11,"fmt","","",53,[[["self"],["formatter"]],["result"]]],[11,"default","","",53,[[],["sendgridapikey"]]],[11,"eq","","",53,[[["self"],["sendgridapikey"]],["bool"]]],[11,"ne","","",53,[[["self"],["sendgridapikey"]],["bool"]]],[11,"fmt","","",53,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",53,[[["d"]],["result"]]],[11,"clone","","",54,[[["self"]],["sqsurl"]]],[11,"fmt","","",54,[[["self"],["formatter"]],["result"]]],[11,"default","","",54,[[],["sqsurl"]]],[11,"eq","","",54,[[["self"],["sqsurl"]],["bool"]]],[11,"ne","","",54,[[["self"],["sqsurl"]],["bool"]]],[11,"fmt","","",54,[[["self"],["formatter"]],["result"]]],[11,"deserialize","","",54,[[["d"]],["result"]]],[11,"fmt","","",55,[[["self"],["formatter"]],["result"]]],[11,"default","","",55,[[],["authdb"]]],[11,"fmt","","",56,[[["self"],["formatter"]],["result"]]],[11,"default","","",56,[[],["aws"]]],[11,"fmt","","",57,[[["self"],["formatter"]],["result"]]],[11,"default","","",57,[[],["awskeys"]]],[11,"clone","","",58,[[["self"]],["bouncelimit"]]],[11,"fmt","","",58,[[["self"],["formatter"]],["result"]]],[11,"default","","",58,[[],["bouncelimit"]]],[11,"clone","","",59,[[["self"]],["bouncelimits"]]],[11,"fmt","","",59,[[["self"],["formatter"]],["result"]]],[11,"default","","",59,[[],["bouncelimits"]]],[11,"fmt","","",60,[[["self"],["formatter"]],["result"]]],[11,"default","","",60,[[],["log"]]],[11,"fmt","","",61,[[["self"],["formatter"]],["result"]]],[11,"default","","",61,[[],["provider"]]],[11,"fmt","","",62,[[["self"],["formatter"]],["result"]]],[11,"default","","",62,[[],["redis"]]],[11,"clone","","",63,[[["self"]],["sender"]]],[11,"fmt","","",63,[[["self"],["formatter"]],["result"]]],[11,"default","","",63,[[],["sender"]]],[11,"fmt","","",64,[[["self"],["formatter"]],["result"]]],[11,"default","","",64,[[],["sendgrid"]]],[11,"fmt","","",65,[[["self"],["formatter"]],["result"]]],[11,"default","","",65,[[],["smtp"]]],[11,"clone","","",66,[[["self"]],["smtpcredentials"]]],[11,"fmt","","",66,[[["self"],["formatter"]],["result"]]],[11,"default","","",66,[[],["smtpcredentials"]]],[11,"clone","","",67,[[["self"]],["socketlabs"]]],[11,"fmt","","",67,[[["self"],["formatter"]],["result"]]],[11,"default","","",67,[[],["socketlabs"]]],[11,"fmt","","",68,[[["self"],["formatter"]],["result"]]],[11,"default","","",68,[[],["sqsurls"]]],[11,"fmt","","",69,[[["self"],["formatter"]],["result"]]],[11,"default","","",69,[[],["settings"]]],[11,"new","","Construct a `Settings` instance, populating it with data from the file system and local environment.",69,[[],["result",["configerror"]]]],[11,"build_rocket_config","","Create rocket configuration based on the environment variable. Defaults to `dev` mode if `FXA_EMAIL_ENV` is not set.",69,[[["self"]],["result",["rocketconfig","rocketconfigerror"]]]],[0,"validate","fxa_email_service","Common validation logic.",N,N],[5,"aws_region","fxa_email_service::validate","Validate an AWS region.",N,[[["str"]],["bool"]]],[5,"aws_access","","Validate an AWS access key.",N,[[["str"]],["bool"]]],[5,"aws_secret","","Validate an AWS secret key.",N,[[["str"]],["bool"]]],[5,"base_uri","","Validate a base URI.",N,[[["str"]],["bool"]]],[5,"email_address","","Validate an email address.",N,[[["str"]],["bool"]]],[5,"env","","Validate env.",N,[[["str"]],["bool"]]],[5,"host","","Validate a host name or IP address.",N,[[["str"]],["bool"]]],[5,"logging_level","","Validate logging level.",N,[[["str"]],["bool"]]],[5,"logging_format","","Validate logging format.",N,[[["str"]],["bool"]]],[5,"provider","","Validate an email provider.",N,[[["str"]],["bool"]]],[5,"sender_name","","Validate a sender name.",N,[[["str"]],["bool"]]],[5,"sendgrid_api_key","","Validate a Sendgrid API key.",N,[[["str"]],["bool"]]],[5,"sqs_url","","Validate an AWS SQS URL.",N,[[["str"]],["bool"]]],[14,"rocket_uri_for_version","fxa_email_service","",N,N],[14,"rocket_uri_for_lbheartbeat","","",N,N],[14,"rocket_uri_for_heartbeat","","",N,N],[14,"rocket_uri_for_handler","","",N,N],[11,"serialize","fxa_email_service::settings","",69,[[["self"],["record"],["str"],["serializer"]],["result"]]]],"paths":[[4,"AppErrorKind"],[3,"AppError"],[3,"BounceRecord"],[4,"BounceType"],[4,"BounceSubtype"],[8,"Db"],[3,"DbClient"],[3,"Bounces"],[3,"DurationError"],[3,"Duration"],[3,"EmailAddress"],[3,"MozlogLogger"],[3,"MessageData"],[3,"MessageDataError"],[3,"Providers"],[3,"Message"],[3,"QueueIds"],[3,"Notification"],[3,"Mail"],[3,"Bounce"],[3,"Complaint"],[3,"Delivery"],[3,"Notification"],[3,"Bounce"],[3,"BouncedRecipient"],[3,"Complaint"],[3,"ComplainedRecipient"],[3,"Delivery"],[4,"NotificationType"],[4,"HeaderValue"],[4,"BounceType"],[4,"BounceSubtype"],[4,"ComplaintFeedbackType"],[3,"Mail"],[3,"Header"],[3,"Queue"],[3,"QueueError"],[8,"Incoming"],[8,"Outgoing"],[8,"Factory"],[3,"Queues"],[3,"Body"],[3,"Email"],[3,"AwsAccess"],[3,"AwsRegion"],[3,"AwsSecret"],[3,"BaseUri"],[3,"DefaultProvider"],[3,"Env"],[3,"Host"],[3,"LoggingLevel"],[3,"LoggingFormat"],[3,"SenderName"],[3,"SendgridApiKey"],[3,"SqsUrl"],[3,"AuthDb"],[3,"Aws"],[3,"AwsKeys"],[3,"BounceLimit"],[3,"BounceLimits"],[3,"Log"],[3,"Provider"],[3,"Redis"],[3,"Sender"],[3,"Sendgrid"],[3,"Smtp"],[3,"SmtpCredentials"],[3,"SocketLabs"],[3,"SqsUrls"],[3,"Settings"]]};
initSearch(searchIndex);
