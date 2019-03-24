use graph_oauth::jwt::Algorithm;
use graph_oauth::jwt::JWT;
use strum::IntoEnumIterator;

// Tests that a JWT algorithm matches the one given and
// that the algorithm is not equal to any other possible matches.
fn test_jwt_validation(key: &str, alg: Algorithm) {
    let mut jwt = JWT::new(key);
    jwt.validate().unwrap();
    let algorithm = jwt.header().unwrap().alg();
    assert_eq!(algorithm, alg);
    for a in Algorithm::iter() {
        if a != algorithm {
            assert_ne!(a, algorithm);
        } else {
            assert_eq!(a, alg);
        }
    }
}

#[test]
fn jwt_alg() {
    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.vgz1gffXdteqASSBz55Yl-cLmTnIv6kDxFMfe6P1BKc";
    test_jwt_validation(key, Algorithm::HS256);

    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.i7MTUwMJJkP8msKx_4zTnykAOT85Vyit0R0XPyHR2fFZu2UIFonFBbLNgvH-Y8Dw";
    test_jwt_validation(key, Algorithm::HS384);

    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.i5Vdk3PhuVleXTwhmqoBkM8NIzw6vRoTcCHml-F49sO0iQSOGechIJllxHxNe0O0U-mNw-chT8VvERY53bQJ6g";
    test_jwt_validation(key, Algorithm::HS512);

    let key = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.TCYt5XsITJX1CxPCT8yAV-TVkIEq_PbChOMqsLfRoPsnsgw5WEuts01mq-pQy7UJiN5mgRxD-WUcX16dUEMGlv50aqzpqh4Qktb3rk-BuQy72IFLOqV0G_zS245-kronKb78cPN25DGlcTwLtjPAYuNzVBAh4vGHSrQyHUdBBPM";
    test_jwt_validation(key, Algorithm::RS256);

    let key = "eyJhbGciOiJSUzM4NCIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.CN9hqUMdVb5LGo06Geb8ap1qYfbJ4rEZIMqTE9gxA2m6GGmsXkznRxzoFpAzQUey9q5HehRTk_-TxYydN3QtFPfrTbAHep7PLhp3XhdvTJ1ok__UBjv4aP6UWTF-Rflr3qeC18LdlM4nyKL7ZwSGDzytWihGod5vn4GAXErUUE4";
    test_jwt_validation(key, Algorithm::RS384);

    let key = "eyJhbGciOiJSUzUxMiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.MejLezWY6hjGgbIXkq6Qbvx_-q5vWaTR6qPiNHphvla-XaZD3up1DN6Ib5AEOVtuB3fC9l-0L36noK4qQA79lhpSK3gozXO6XPIcCp4C8MU_ACzGtYe7IwGnnK3Emr6IHQE0bpGinHX1Ak1pAuwJNawaQ6Nvmz2ozZPsyxmiwoo";
    test_jwt_validation(key, Algorithm::RS512);

    let key = "eyJhbGciOiJQUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.P9_X1ctIxnnoUpKSWpYw3rF62e-d8LXe3sETuLn4Lhigw5OQhi-mBBKoBMneHy4kimS84zxnMby0FYo9wKM3I3pEg8Qrz0Q00tNhKCwOnZ7Q-e86sW1luK1z82tufF-sZ9_BY_LGQsym0lQmQaHFzLmEDXnOzWsjUThHGVJTI64";
    test_jwt_validation(key, Algorithm::PS256);

    let key = "eyJhbGciOiJQUzM4NCIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.oywIg-I6w59yw9jiPxewn5n2BhrD7hSifWSmzFKGBMPEMd0qweVNjlyxu2TodunPzlh49OW8QA0ygNRL9VQrWA3GXzb5FubNF4s7Y15QePx52anlvebzihx5-hR0UhKbVC0UODwYNMiY-v0L7iMbT9UvuSj0GAuZMxndo2Y2VFQ";
    test_jwt_validation(key, Algorithm::PS384);

    let key = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.tyh-VfuzIxCyGYDlkBA7DfyjrqmSHu6pQ2hoZuFqUSLPNY2N0mpHb3nk5K17HWP_3cYHBw7AhHale5wky6-sVA";
    test_jwt_validation(key, Algorithm::ES256);

    let key = "eyJhbGciOiJFUzM4NCIsInR5cCI6IkpXVCIsImtpZCI6ImlUcVhYSTB6YkFuSkNLRGFvYmZoa00xZi02ck1TcFRmeVpNUnBfMnRLSTgifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.cJOP_w-hBqnyTsBm3T6lOE5WpcHaAkLuQGAs1QO-lg2eWs8yyGW8p9WagGjxgvx7h9X72H7pXmXqej3GdlVbFmhuzj45A9SXDOAHZ7bJXwM1VidcPi7ZcrsMSCtP1hiN";
    test_jwt_validation(key, Algorithm::ES384);
}

#[test]
#[should_panic]
fn invalid_jwt_hs() {
    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.vgz1gffXdteqASSBz55Yl-cLmTnIv6kDxFMfe6P1BKc";
    test_jwt_validation(key, Algorithm::HS384);
}

#[test]
#[should_panic]
fn invalid_jwt_rs() {
    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.vgz1gffXdteqASSBz55Yl-cLmTnIv6kDxFMfe6P1BKc";
    test_jwt_validation(key, Algorithm::RS256);
}