// Postman tests

// Login
pm.test("authTest", function () {
    pm.response.to.have.status(200);
});

pm.test("cookieTest", function () {
    pm.expect(pm.cookies.has('auth')).to.be.true;
});

// Auth
pm.test("userAuthTest", function () {
    pm.response.to.have.status(200);
    responseBody = responseBody.slice(1, -1);
    pm.collectionVariables.set("user", responseBody);
});

// User data
