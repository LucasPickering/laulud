# If you update this, make sure you update the .nvmrc too
FROM node:16.13.1-alpine AS builder

WORKDIR /app/ui
COPY ./ui/ .
# Copy the actual schema in *second* so it overwrites the symlink
COPY ./api/schema.graphql .

RUN npm install
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/ui/build /app/laulud
COPY ./ui/nginx.conf /etc/nginx/nginx.conf
