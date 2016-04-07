# Small interactive server

This is a simple interactive response server, where you can change the response from a REPL.

## How to use

To start the server, from this folder, execute the following:

```bash
ruby -W0 server.rb
```

It will start a REPL, and you can use some of the following methods:

```
irb> ok
# will set the server to respond with 200
irb> error
# will set the server to respond with 500
irb> not_found
# will set the server to respond with 404
irb> dead
# will just close the connection
irb> alive
# will respond again
irb> speed 1
# will take 1 second to respond
irb> exit
# will exit the application
```
