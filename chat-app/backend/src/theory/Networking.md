# Networking

## Basic json authentication request

<mark>

curl -X POST http://127.0.0.1:8080/login \
 -H "Content-Type: application/json" \
 -d '{"username": "admin", "password": "password}'

- -X POST -> specifies a POST request.
- -H "Contente-Type: application/json" -> Tells the server that we're sending JSON.
- -d "..." -> Sends JSON data in the request body.

curl -X GET http://127.0.0.1:8080/protected-rooute \
-H "Authorization: Beare ${token}"

`Sending credentials with basic authentication`
curl -X Get http://127.0.0.1:8080/protected-route \  
 -u admin:password

</mark>
