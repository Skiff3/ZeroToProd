use actix_web::{HttpResponse, web};
use sqlx::{PgConnection, PgPool};
use tracing::{error, Instrument};
use sqlx::types::uuid;
use chrono::Utc;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;//---->??
//use unicoe_segmentation::UnicodeSegmentation;
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,// this is email of form data.
    name: String
}

pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber,String>{
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;
    Ok(NewSubscriber{email,name})
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form,pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>,pool:web::Data<PgPool>,) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        //Ok(subscriber) => subscriber,
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    // let name = match SubscriberName::parse(form.0.name) {
    //     Ok(name) => name,
    //     Err(_) => return
    //     HttpResponse::BadRequest().finish(),
    // };
    // let email = match SubscriberEmail::parse(form.0.email){
    //     Ok(email) => email,
    //     Err(_) => return
    //     HttpResponse::BadRequest().finish(),
    // };
    // //let new_subscriber = crate::domain::SubscriberName(form.name.clone());
    // let new_subscriber = NewSubscriber{
    //     email,
    //     name,
    // };
    // if !is_valid_name(&form.name) {
    //     return HttpResponse::BadRequest().finish()
    // }

    match insert_subscriber(&pool, &new_subscriber).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }//there
}

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/','(',')','"','<','>','\\','{','}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}//rends here

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber,pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    //form: &FormData,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {//the result returns and error
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),//inner_ref()??
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e|{
            tracing::error!("Failed to execute query: {:?}",e);
        e
    })?;
    Ok(())
}

impl SubscriberName{
    pub fn inner_ref(&self) -> &str{
        &self.0
    }
}

impl TryFrom<FormData> for NewSubscriber{
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self{ email, name})
    }
}
//
// pub fn do_something_with_a_string_slice<T: AsRef<str>>(s: T){
//     let s = s.as_ref();
// }
//     let request_id = Uuid::new_v4();//Uuid is used to generate a random id
//     let request_span = tracing::info_span!(
//         name = "Adding a new subscriber.",//tracing when a new subscriber is added.
//         skip(form,pool),
//         fields(
//             %request_id,  //request_id of the subscriber to track the error better in the log trace
//             subscriber_email = %form.email, //e-mail of the subscriber in the log trace
//             subscriber_name = %form.name//name of the subscriber in the log trace
//         )
//
//         // &a-->lj.23999909887,9.99898989,3.45888890,2.788889988=>{
//         //     &b,&b,&v,&c(0.999,0.88,0.898,0.8876)
//         //     #90099998.kjjj99999&88
//         // }
//     );
//     let_request_span_guard = request_span.enter();
//     let query_span = tracing::info_span!("Saving new subscriber details in the database");
//     match sqlx::query!(
//         r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
//         Uuid::new_v4(),//Uuid is used to generate random id for the user in the table
//         form.email,//e-mail of the subscriber in the query
//         form.name,//name of the subscriber in the query
//         Utc::now()//timestamp when the query was created.
//     )
//         .execute(pool.get_ref())
//         //First we attach the instrumentation, then we have to wait it out.
//         .instrument(query_span)
//         .await
//     {
//         Ok(_) => {
//         //tracing::info!("request_id {} - New subscriber details have been saved",request_id);
//         HttpResponse::Ok().finish()
//         },
//         Err(e) => {
//             tracing::error!("request_id {} - Failed to execute query: {:?}",request_id,e);//log dependency is used to display errors.
//             //println!("Failed to execute query: {}",e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }