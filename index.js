async function create_auth_client(domain, client_id) {
    auth0 = await createAuth0Client({
        domain,
        client_id,
    });
}
