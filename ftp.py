from pyftpdlib.authorizers import DummyAuthorizer
from pyftpdlib.handlers import FTPHandler
from pyftpdlib.servers import FTPServer

# Define the root directory for the FTP server
ftp_root = "/home/noahdominic/Developer/journey2"

# Create an authorizer with a single user (anonymous user disabled)
authorizer = DummyAuthorizer()
authorizer.add_user("username", "password", ftp_root, perm="elradfmw")

# Define the FTP handler
handler = FTPHandler
handler.authorizer = authorizer

# Create the FTP server
server = FTPServer(("0.0.0.0", 21), handler)

# Start the server
server.serve_forever()

