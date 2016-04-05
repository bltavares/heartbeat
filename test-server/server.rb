require 'socket'
require 'irb'
require 'irb/completion'

OK = "HTTP/1.1 200 OK\r\n"
NOT_FOUND = "HTTP/1.1 404 Not Found\r\n"
ERROR = "HTTP/1.1 500 Internal Server Error\r\n"

@@live = true
@@response = OK

port = ENV.fetch('PORT', 7125)
puts "Starting server at localhost:#{port}"
webserver = TCPServer.new('127.0.0.1', port)
server = Thread.new do
  while (session = webserver.accept)
    session.print @@response if @@live
    session.close
  end
end


def ok
  @@response = OK
end

def error
  @@response = ERROR
end

def not_found
  @@response = NOT_FOUND
end

def alive
  @@live = true
end

def dead
  @@live = false
end

IRB.start
server.exit
