# Azero.gg

## Todo

- [ ] Setup cargo: 
    * serenity
    * uuid
    * warp (web server)
- [ ] Deployment: 
    * Bot in localhost
    * Web server in localhost
    * Webapp in localhost
- [ ] 


@Luca:
*  modulo: webServer
    * Creare rotta '/auth?uuid=<UUID>'
    * Creare funzione per lo spawn di un web server warp su una determinata porta
    * luca riceve da Nic GuildID e DiscordID, si recupera il discord token da env e assegna il ruolo il cui ID è in env allo user

@Nicolas:
*  modulo: frontend
    * Collegare wallet aleph zero: https://docs.alephzero.org/aleph-zero/build/aleph-zero-signer-integration
    * pacchetto azero id resolver: https://github.com/azero-id/resolver utile per capire come querare un 
        contratto pallet su aleph zero (ad esempio per fetchare il balance)
    * effettuare una richiesta HTTP POST al webserver per associare il discordId con l'accountId:
        ```json
        {
            "data": {
                "discordId": "1298467219461892469",
                "accountId": "D5AF12412F215987ABCABB9357124986ACCAC124174"
            }
        }
        ```


@Federico:
* modulo: discord
    <!-- * Creare funzione per lo spawn di un server discord -->
    <!-- * Bot deve essere invitato alla chat -->
    * "autocomplete" per pescare token da json file (USDC=>0x1234,...)  
    <!-- * Se rich embed con bottone non è possibile magari facciamo slash command \auth disponibile accessibile a tutti i profili aventi il ruolo non-verificato -->
    <!-- * slash command config accessibile solo a chi ha il ruolo admin -->
    <!-- * Bot deve avere pulsante per autenticare l'utente: ascolta gli ingressi e rispondi ai tipi che entrano -->
    <!-- * bot che entra crea ruolo "authenticated". Questo ruolo sarà assegnato all'utente se l'utente riesce ad autenticarsi -->
    <!-- * hashmap guildID -> Config of that server -->
    <!-- * understand how to trigger event handler externally -->







# Tips:
- Inkathon boilerplate has the widget for connecting the wallet
- Discord bot click button: <https://serenity-rs.github.io/serenity/current/serenity/model/application/component/struct.Button.html>