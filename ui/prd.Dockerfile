# If you update this, make sure you update the .nvmrc too
FROM node:14.15.0-alpine AS builder

COPY ./api/schema /app/api/schema/
COPY ./ui/ /app/ui/
WORKDIR /app/ui
RUN npm install
RUN npm run build

FROM alpine:latest
COPY --from=builder /app/ui/build /app/static
