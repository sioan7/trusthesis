# Keri tutorial series - `kli` sign and verify

References:

- https://kentbull.com/2023/01/27/keri-tutorial-series-kli-sign-and-verify-with-heartnet/

## Docker install

```bash
docker run --rm -it gleif/keri /bin/bash
```

## Instructions

### Create configuration and keystore dirs

```bash
mkdir -p /keri/heartnet/keri/cf
mkdir /keri/{allie,brett}
export KERI_CONFIG_DIR=/keri/heartnet

echo '{"dt": "2022-01-20T12:57:59.823350+00:00","iurls": ["http://127.0.0.1:5642/oobi/BBilc4-L3tFUnfM_wJr4S4OJanAv_VmF_dJNN6vkf2Ha/controller","http://127.0.0.1:5643/oobi/BLskRTInXnMxWaGqcpSyMgo0nYbalW99cGZESrz3zapM/controller","http://127.0.0.1:5644/oobi/BIKKuvBwpmDVA4Ds-EpL5bt9OqPzWPja2LigFYZN2YfX/controller"]}' \
     > /keri/heartnet/keri/cf/allie-witness-oobis.json

echo '{"dt": "2022-01-20T12:57:59.823350+00:00","iurls": ["http://127.0.0.1:5645/oobi/BM35JN8XeJSEfpxopjn5jr7tAHCE5749f0OobhMLCorE/controller","http://127.0.0.1:5646/oobi/BIj15u5V11bkbtAxMA7gcNJZcax-7TgaBMLsQnMHpYHP/controller","http://127.0.0.1:5647/oobi/BF2rZTW79z4IXocYRQnjjsOuvFUQv-ptCf8Yltd7PfsM/controller"]}' \
     > /keri/heartnet/keri/cf/brett-witness-oobis.json

echo '{"transferable": true,"wits": ["BBilc4-L3tFUnfM_wJr4S4OJanAv_VmF_dJNN6vkf2Ha","BLskRTInXnMxWaGqcpSyMgo0nYbalW99cGZESrz3zapM","BIKKuvBwpmDVA4Ds-EpL5bt9OqPzWPja2LigFYZN2YfX"],"toad": 3,"icount": 1,"ncount": 1,"isith": "1","nsith": "1"}' \
     > /keri/allie/magic-pencil.json

echo '{"transferable": true,"wits": ["BM35JN8XeJSEfpxopjn5jr7tAHCE5749f0OobhMLCorE","BIj15u5V11bkbtAxMA7gcNJZcax-7TgaBMLsQnMHpYHP","BF2rZTW79z4IXocYRQnjjsOuvFUQv-ptCf8Yltd7PfsM"],"toad": 3,"icount": 1,"ncount": 1,"isith": "1","nsith": "1"}' \
     > /keri/brett/secret-speaker.json
```

### Start witness network

```bash
kli witness demo &
```

### Initialize keystores

```bash
export ALLIE_SALT="$(kli salt)"
kli init \
      --name allie_ks \
      --base /keri/allie  \
      --nopasscode \
      --salt ${ALLIE_SALT} \
      --config-dir ${KERI_CONFIG_DIR} \
      --config-file allie-witness-oobis
```

```bash
export BRETT_SALT="$(kli salt)"
kli init \
      --name brett_ks \
      --base /keri/brett  \
      --nopasscode \
      --salt ${BRETT_SALT} \
      --config-dir ${KERI_CONFIG_DIR} \
      --config-file brett-witness-oobis
```

### Create KERI identifiers by making an inception event

```bash
kli incept \
      --name allie_ks \
      --base /keri/allie \
      --alias magic-pencil \
      -f /keri/allie/magic-pencil.json

export ALLIE_PREFIX=
```

```bash
kli incept \
      --name brett_ks \
      --base /keri/brett  \
      --alias secret-speaker \
      -f /keri/brett/secret-speaker.json
 
export BRETT_PREFIX=
```

### Connect the two KERI identifiers using OOBIs (direct service discovery)

```bash
kli oobi generate \
      --name allie_ks \
      --base /keri/allie \
      --alias magic-pencil \
      --role witness

export magic_pencil_oobi=
```

```bash
kli oobi generate \
      --name brett_ks \
      --base /keri/brett \
      --alias secret-speaker \
      --role witness

export secret_speaker_oobi=
```

### OOBI resolution (discovery)

```bash
kli oobi resolve \
    --name brett_ks \
    --base /keri/brett \
    --oobi-alias magic-pencil \
    --oobi "${magic_pencil_oobi}"
```

```bash
kli oobi resolve \
      --name allie_ks \
      --base /keri/allie \
      --oobi-alias secret-speaker \
      --oobi "${secret_speaker_oobi}"
```

### Increase the trust level with MFA challenge phrases

#### Generate challenge phrase

```bash
allie_words="$(kli challenge generate --out string)"

brett_words="$(kli challenge generate --out string)"
```

#### Prepare challenge response

```bash
kli challenge respond \
      --name allie_ks \
      --base /keri/allie \
      --alias magic-pencil \
      --recipient secret-speaker \
      --words "${allie_words}"

kli challenge verify \
      --name brett_ks \
      --base /keri/brett \
      --alias secret-speaker \
      --signer magic-pencil \
      --words "${allie_words}"
```

```bash
kli challenge respond \
      --name brett_ks \
      --base /keri/brett \
      --alias secret-speaker \
      --recipient magic-pencil \
      --words "${brett_words}"

kli challenge verify \
      --name allie_ks \
      --base /keri/allie \
      --alias magic-pencil \
      --signer secret-speaker \
      --words "${brett_words}"
```

### Write and sign the love letter

```bash
echo '{"love_letter": "well, hello there, honey. Happy Valentines :*"}' > /keri/heartnet/love-letter.json

kli sign \
      --name allie_ks \
      --base /keri/allie \
      --alias magic-pencil \
      --text @/keri/heartnet/love-letter.json 
```

### Verify the love letter signature

```bash
kli verify \
      --name brett_ks \
      --base /keri/brett \
      --alias secret-speaker \
      --prefix $ALLIE_PREFIX \
      --text @/keri/heartnet/love-letter.json \
      --signature $ALLIE_SIGNATURE
```

### Write and sign the love reply

```bash
echo '{"love_letter": "Hey sweetie, I got your letter! <3 <3"}' > /keri/heartnet/love-reply.json

kli sign \
      --name brett_ks \
      --base /keri/brett \
      --alias secret-speaker \
      --text @/keri/heartnet/love-reply.json
```

### Verify the love reply

```bash
kli verify \
    --name allie_ks \
    --base /keri/allie \
    --alias magic-pencil \
    --prefix $BRETT_PREFIX \
    --text @/keri/heartnet/love-reply.json \
    --signature $BRETT_SIGNATURE
```