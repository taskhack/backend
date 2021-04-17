# API Documentation

## /api/v1/users/<email>

### [GET] /
Returns information about a user

### [POST] /
Sets information about a user


### [GET] /login
Logs a user in ("gets" a token)

### [POST] /register
Registers a user

### [POST] /projects
Retruns a list of projects the user is a part of

## /api/v1/projects/<projectid>

### [GET] /
Returns information about a project

### [POST] /
Updates project details or creates a project

### /<projectid>/tasks/<taskid>

#### [GET] /
Gets task information, including comments

#### [POST] /
Updates task information

#### [POST] /create
Creates a task, with task identifier taskid

#### [POST] /comment
Creates a comment on that task ID
