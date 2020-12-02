FROM rust:alpine

COPY . /bin/add-it

WORKDIR /bin/add-it

RUN chmod +x add-it

CMD [ "/bin/add-it" ]

# FROM rust:alpine as builder

# WORKDIR "/add-it/user-app"

# RUN rustup default nightly \
#  && rustup update
 
# COPY filename.sh .

# # copy dependency files 
# COPY Cargo.toml Cargo.toml

# # get user application dependencies
# RUN cargo fetch 

# COPY Rocket.toml Rocket.toml

# #copy user code
# COPY src ./src

# ENV ROCKET_ENV=production

# # build for release
# RUN cargo build --release

# FROM rust:1.37-slim-stretch

# RUN useradd rust

# WORKDIR "/project/user-app"

# # get files and built binary from previous image
# COPY --from=builder /project/user-app/filename.sh /project/user-app/Cargo.toml /project/user-app/Rocket.toml /project/user-app/target/release/ ./

# RUN chmod +x filename.sh \
#  && chown rust:rust $(./filename.sh)

# USER rust

# EXPOSE 8000

# CMD ["bash", "-c", "./$(./filename.sh)"]