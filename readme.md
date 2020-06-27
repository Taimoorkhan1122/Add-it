# Add-it endpoint api in Rust 
- Built on Rocket this add-it api takes querry parameter and returns adding number defined.
-for example    `https://wwww.someurl.com/add-it/10` <br>
the response will be **15**
- If user enters a value Other than a number it'will return Error with querry parameter provdied by the user
-for example    `https://wwww.someurl.com/add-it/Some other text` <br>
the response will be **Not a Number: Some other text**

## that's it!
