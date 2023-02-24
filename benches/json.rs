use binary_extract;
use criterion::{criterion_group, criterion_main, Criterion};
use json;

fn native_extract(input: &str) {
    let obj = json::parse(input).unwrap();
    let _ = &obj["projectId"];
}

fn extract(input: &str) {
    let _ = binary_extract::extract(input, "projectId").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = "{\"properties\":{\"selected\":\"2\",\"lastName\":\"\",\"username\":\"someone\",\"category\":\"Wedding Venues\",\"firstName\":\"\",\"product\":\"planner\",\"location\":\"\",\"platform\":\"ios\",\"email\":\"someone@yahoo.com\",\"member_id\":\"12312313123123\",\"filtered\":\"false\",\"viewed\":3},\"projectId\":\"foobarbaz\",\"userId\":\"123123123123123\",\"sessionId\":\"FF8D19D8-123123-449E-A0B9-2181C4886020\",\"requestId\":\"F3C49DEB-123123-4A54-BB72-D4BE591E4B29\",\"action\":\"Track\",\"event\":\"Vendor Category Viewed\",\"timestamp\":\"2014-04-23T20:55:19.000Z\",\"context\":{\"providers\":{\"Crittercism\":false,\"Amplitude\":false,\"Mixpanel\":false,\"Countly\":false,\"Localytics\":false,\"Google Analytics\":false,\"Flurry\":false,\"Tapstream\":false,\"Bugsnag\":false},\"appReleaseVersion\":\"2.3.1\",\"osVersion\":\"7.1\",\"os\":\"iPhone OS\",\"appVersion\":\"690\",\"screenHeight\":480,\"library-version\":\"0.10.3\",\"traits\":{\"lastName\":\"\",\"product\":\"planner\",\"member_id\":\"123123123123123\",\"firstName\":\"\",\"email\":\"someone@yahoo.com\",\"platform\":\"ios\",\"username\":\"someone\"},\"screenWidth\":320,\"deviceManufacturer\":\"Apple\",\"library\":\"analytics-ios\",\"idForAdvertiser\":\"1323232-A0ED-47AB-BE4F-274F2252E4B4\",\"deviceModel\":\"iPad3,4\"},\"requestTime\":\"2014-04-23T20:55:44.211Z\",\"version\":1,\"channel\":\"server\"}";

    c.bench_function("native json", |b| b.iter(|| native_extract(&input)));
    c.bench_function("binary_extract", |b| b.iter(|| extract(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
