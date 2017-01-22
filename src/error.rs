error_chain! {

    errors {
       InvalidForm(model: String, reason: String) {
           description("the model is invalid")
           display("the model for {0} is invalid due to: {1}", model, reason) 
       }
    }
}
