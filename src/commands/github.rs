https://github.com/tmcarr/testbot


#[command]
#[aliases("source")]
fn github(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let value = &args.message();
    db.set(&String::from(&msg.author.name), value).unwrap();
    let github = "'https://gihub.con/tmcarr/testbot";
    let _ = msg.channel_id.say(
        &ctx.http,
        &format!("My code is at: {}", &github),
    );
    Ok(())
}