#!/usr/bin/env nu

git log "--pretty=format:%h<nu>%aN<nu>%s<nu>%aD" | lines | split-column "<nu>" sha1 committer desc merged_at | first 10

git log "--pretty=format:%h<nu>%aN<nu>%s<nu>%aD" | { lines
 } | split-column "<nu>" sha1 committer desc merged_at {
| first 10
 }


git log "--pretty=format:%h<nu>%aN<nu>%s<nu>%aD" | {
   lines
  | split-column "<nu>" sha1 committer desc merged_at 
  | first 10
 }

git log "--pretty=format:%h<nu>%aN<nu>%s<nu>%aD" {
  | lines
  | split-column "<nu>" sha1 committer desc merged_at 
  | first 10
 }

# This was suggested by Jonathan.
do {
  git log "--pretty=format:%h<nu>%aN<nu>%s<nu>%aD"
  | lines
  | split-column "<nu>" sha1 committer desc merged_at 
  | first 10
}

