AWS Lambda Function example
Build & Deploy
Step1.
**Install cargo-lambda**
Step2.
Build the function with **cargo lambda build --release**
Step3.
Deploy the function to AWS Lambda with **cargo lambda deploy --iam-role YOUR_ROLE**

example(** cargo lambda deploy --iam-role arn:aws:iam::905418486995:role/easyrust --binary-name lambdafn** )
Build for ARM 64
Build the function with cargo lambda build --release --arm64



중요 ! this is IAM role 을 먼저 등록하셔야 됩니다! 
그래서 ARN을 가져와서 YOUR_ROLE 에 기입해줍니다.
https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/roles/details/easyrust?section=permissions
