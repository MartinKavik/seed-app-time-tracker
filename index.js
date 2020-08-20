import init from '/pkg/package.js';

let auth0 = null;

window.init_auth = async (domain, client_id) => {
    auth0 = await createAuth0Client({
        domain,
        client_id,
    });

    const query = window.location.search;
    if (query.includes("code=") && query.includes("state=")) {
      await auth0.handleRedirectCallback();
    }

    if (await auth0.isAuthenticated()) {
        return await auth0.getUser();
    }
}

window.redirect_to_sign_up = async () => {
    await auth0.loginWithRedirect({
        redirect_uri: window.location.origin,
        screen_hint: "signup"
    });
}

window.redirect_to_log_in = async () => {
    await auth0.loginWithRedirect({
        redirect_uri: window.location.origin,
    });
}

window.logout = () => {
    auth0.logout({
        returnTo: window.location.origin
    });
}

init('/pkg/package_bg.wasm');
