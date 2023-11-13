# rinora-mud

The RinoraMUD

## Contributing

- Need to install PGSQL CLI Tools (and optionally pgAdmin 4)
- Add the `bin` and `lib` folder of the new PGSQL installation to your system PATH

## Workflow

On top of Bevy's `First`, `PreUpdate`, `Update`, `PostUpdate` and `Last`, the game system has the following system sets 
which are scheduled to run in the following order, allowing all systems and commands to be completed per step:

- `Network` - Handles processing incoming and outgoing data. Most of the network side runs on a separate thread, but 
there are some exclusive systems that move data between the threads and the rest of the system. You normally won't need 
to add anything here unless you're supporting additional protocols.

- `Command` - Processes incoming commands from the network layer. This is both the typical text input from the user, 
and GMCP commands from the client. Commands have a structure where they check if the command can run and, if it can, it 
runs the command, which should at most validate state and get the information needed for the game world to execute the 
rest of the command, then issue events for the next step to process.

- `Account` - The account commands systems are checked separately from all the game commands in order to better support 
how many of the account-based commands aren't keyword based. Also, this allows us to log a character in here, and in 
later steps have the character appear as all part of the same game cycle.

- `Pre` - These systems run before the main game world systems do. They are meant to update the game work to the state 
it's expected to be after all the appropriate commands have ran. This is to help avoid relying on the commands to do 
the work instead: We want to make the system run in parallel as much as possible.

- `Game` - All the primary game systems. This is where the majority of the logic that updates the game state should be.

- `Post` - Systems that you wish to run after the game world has updated. Here, you might resolve room events, and find 
who should be notified of them. Basically, this step converts what happened in the game into what the users sees, or 
notifies game entities appropriately.

- `Cleanup` - This step is meant to clean up any resources, queries, or connections. Here, for example, a character who 
has logged out will have their state saved to the database.

- `Debug` - This layer is specifically meant for debugging information. Here you can route events to logs, streams, or 
whatever you need. You can also use this step to enhance admin output with additional detail, for instance.

- `Output` - This converts all the `TextEvent` and `DataEvents` into the appropriate network code to be sent out by the 
server. You likely won't need to mess with this unless you're supporting additional protocols.

## Migrations

We use [Diesel](https://diesel.rs/) to handle migrations. In order to create a migration, you need to install the Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Once installed, you can run the migrations like so, assuming you're using the default development database:

```bash
diesel migration redo --database-url=postgresql://dev:dev@localhost/rinoramud
```
