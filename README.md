
# Progress ReadMe file

This file is going to posess all what we have learnt in a particular commit this file content is related to...


*Commit: Built a basic logging service*


**In this commit we built a basic Logging Serive to log the request on console.**

(1) We used Pinned Box future to hold the response of call method of Service trait implementation.

(2) Future needed to have Send trait as executer manages the futures of services in different traits for optimizations.