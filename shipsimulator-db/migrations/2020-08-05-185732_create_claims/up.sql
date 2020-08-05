 -- Your SQL goes here
CREATE TABLE claims (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL
);

CREATE TABLE claim_user (
    id SERIAL PRIMARY KEY,
    claim_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
	    REFERENCES users(id),
    CONSTRAINT fk_claim
        FOREIGN KEY(claim_id)
        REFERENCES claims(id)
);

INSERT INTO claims(name)
	VALUES ('Developer');

INSERT INTO claims(name)
	VALUES ('Admin');

INSERT INTO claims(name)
	VALUES ('Moderator');

INSERT INTO claims(name)
	VALUES ('Customer');