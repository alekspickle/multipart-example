# Multipart-example
Example of multipart request implementation.

the goal here to make it run from single command:

```bash
docker-compose up
```


## mock bash script
```bash
curl -v -F upload=@upload_example.png http://localhost:3000/multipart_image
```