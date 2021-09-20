alter table invitations
add column reset_request_id UUID NULL;

ALTER TABLE invitations
    ADD CONSTRAINT fk_invitations_reset_pw
    FOREIGN KEY (reset_request_id) 
    REFERENCES reset_requests (id);
