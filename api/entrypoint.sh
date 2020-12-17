#!/bin/sh

# Read each docker secret file into an env var
for f in ${SECRETS_DIR-"/run/secrets/"}*; do
    if [ -e "$f" ]; then
        # Strip any prefixes before before "keskne_", then convert to upper case
        var_name=$(echo $f | sed -E 's/^.*?keskne_(.*)$/\1/' | tr '[:lower:]' '[:upper:]')
        echo "Reading \"$f\" into \"$var_name\""
        export $var_name=$(cat $f) # Load the secret value
    fi
done

# Map to the variables we actually need
export ROCKET_SECRET_KEY=${LAULUD_SECRET_KEY}
export ROCKET_SPOTIFY_CLIENT_ID=${LAULUD_SPOTIFY_CLIENT_ID}
export ROCKET_SPOTIFY_CLIENT_SECRET=${LAULUD_SPOTIFY_CLIENT_SECRET}

exec $@
