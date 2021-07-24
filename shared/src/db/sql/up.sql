CREATE TABLE commits (
  rev_hash VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE branches (
  id NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  head_rev VARCHAR NOT NULL,
  FOREIGN KEY (head_rev)
    REFERENCES commits(rev_hash)
);

CREATE TABLE pull_requests (
  id INT NOT NULL PRIMARY KEY,
  head_rev VARCHAR NOT NULL,
  base_branch_id INT NOT NULL,
  base_rev VARCHAR NOT NULL,     -- Revision which review was performed against
  pr_review_status INT NOT NULL, -- e.g. In Progress, Queued
  FOREIGN KEY (head_rev)
    REFERENCES commits(rev_hash),
  FOREIGN KEY (base_branch_id)
    REFERENCES commits(rev_hash)
  FOREIGN KEY (pr_review_status)
    REFERENCES commits(rev_hash)
);

-- This will be used for determing if queued jobs should be removed
-- as commits that no longer exist should be removed, as well as
-- related queued builds
CREATE TABLE pr_commits (
  id INT NOT NULL PRIMARY KEY,
  pull_request_id INT NOT NULL,
  commit_rev VARCHAR NOT NULL
);

CREATE TABLE event_status (
  id INT NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL                 -- e.g. Unknown, Queued, Success, Failed
);

CREATE TABLE platforms (
  id INT NOT NULL PRIMARY KEY,
  platform VARCHAR NOT NULL              -- e.g. x86_64-linux
);

CREATE TABLE build_status (
  id INT NOT NULL PRIMARY KEY,
  status_name VARCHAR NOT NULL              -- e.g. Queued, Success, Failed
);

CREATE TABLE pr_status (
  id INT NOT NULL PRIMARY KEY,
  status_name VARCHAR NOT NULL
);

CREATE TABLE drvs (
  drv_path VARCHAR NOT NULL PRIMARY KEY, -- e.g. kmgpsrjdqs4f1j8i46nm3yrg72140063-python3-3.8.8 in /nix/store/kmgpsrjdqs4f1j8i46nm3yrg72140063-python3-3.8.8.drv
  attribute VARCHAR NOT NULL,            -- e.g. python38
  previous_drv VARCHAR NULL,             -- e.g. yi9ndyp3pps70mw1kasiv6zaqgrmi057-python3-3.8.8
  platform_id INT NOT NULL,              -- e.g. x86_64-linux
  commit_rev_hash VARCHAR NOT NULL,      -- e.g. 0120f5e5b58e88b783b8149ed271a002e19babd3
  build_status_id INT NOT NULL,          -- e.g. Unknown, Queued, Success, Failed
  FOREIGN KEY (commit_rev_hash)
    REFERENCES commits(rev_hash),
  FOREIGN KEY (platform_id)
    REFERENCES platforms(id),
  FOREIGN KEY (previous_drv)
    REFERENCES drvs(drv_path),
  FOREIGN KEY (build_status_id)
    REFERENCES build_status(id)
);

INSERT INTO build_status
VALUES
  (1, 'Unknown'),
  (2, 'Queued'),
  (3, 'In Progress'),
  (4, 'Success'),
  (5, 'Failed');

INSERT INTO pr_status
VALUES
  (1, 'Unknown'),
  (2, 'Queued'),
  (3, 'In Progress'),
  (4, 'Closed'),
  (5, 'Success'),
  (6, 'Failed');