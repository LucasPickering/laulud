# If you update this, make sure you update the .nvmrc too
FROM node:14.15.0-alpine AS builder

COPY schema.ts /app/
COPY ./ui/ /app/ui/
WORKDIR /app/ui
RUN npm install
RUN npm run build --mode=production

FROM alpine:latest
COPY --from=builder /app/ui/build /app/static