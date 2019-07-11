# Example of multipart request implementation.

the goal here to make it run from single command:

```bash
docker-compose up
```

## Multipart example
#### pick some picture and either rename it to 'upload_example.png' or change the upload path in the curl command
```bash
curl -v -F upload=@upload_example.png https://my-rust-showoff-server.herokuapp.com/multipart_image
```
