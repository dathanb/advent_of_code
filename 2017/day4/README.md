I originally started writing this in SQLite, but decided to switch to Postgres to take advantage of its array functions.

So to run this code you first have to create a Postgres server. Something like:
```
brew install postgresql
initdb -D db -U `whoami`
postgres -D db/
```

Then you can run the code with `./run`
