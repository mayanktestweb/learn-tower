
# Progress ReadMe file

This file is going to posess all what we have learnt in a particular commit this file content is related to...


*Commit: Understanding Service making*


**So in this commit we saw how can we create a Custom Service using Service trait from tower. In our example we built BasicService. Following is the important stuff to notice...**

(1) the Request type in Service<Request> can be of any type. In our case it was String. We can have multiple implementations as Request is generic. So we may have also defined 

```
impl Serivce<String> for MyService {...  // It's valid
impl Serivce<u32> for MyService {...     // and so is it...
```

(2) the Response though is not generic instead it's associated type. Thus for 
every particular implementation there is a fixed Response Type...

```
impl Service<String> for MyService {
    type Response = u32;   // now this response is bound with Serivce<String>
    ....
```

(3) Mind the fact that the type Future in Serivce defination has to implement Future trait from standard library. In our case it's MyFuture
```
pub struct MyFuture{...
impl Future for MyFuture {
    type Output = Result<ServiceResponseType, SerivceErrorType>
```
yet another important thing is that this custom Future has to have Output type of Result with ServiceResponseType and SerivceErrorType...