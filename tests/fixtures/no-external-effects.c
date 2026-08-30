#define _GNU_SOURCE

#include <fcntl.h>
#include <spawn.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

static void block_effect(const char *operation) __attribute__((noreturn));

static void block_effect(const char *operation) {
    const char *path = getenv("ITE_SIDE_EFFECT_LOG");
    if (path != NULL) {
        int descriptor = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
        if (descriptor >= 0) {
            (void)write(descriptor, operation, __builtin_strlen(operation));
            (void)write(descriptor, "\n", 1);
            (void)close(descriptor);
        }
    }
    _exit(190);
}

int socket(int domain, int type, int protocol) {
    (void)domain;
    (void)type;
    (void)protocol;
    block_effect("network socket opened");
}

int connect(int descriptor, const struct sockaddr *address, socklen_t length) {
    (void)descriptor;
    (void)address;
    (void)length;
    block_effect("network connection attempted");
}

pid_t fork(void) {
    block_effect("child process forked");
}

pid_t vfork(void) {
    block_effect("child process vforked");
}

int execve(const char *path, char *const arguments[], char *const environment[]) {
    (void)path;
    (void)arguments;
    (void)environment;
    block_effect("child process executed");
}

int execveat(int directory, const char *path, char *const arguments[], char *const environment[], int flags) {
    (void)directory;
    (void)path;
    (void)arguments;
    (void)environment;
    (void)flags;
    block_effect("child process executed");
}

int posix_spawn(pid_t *process, const char *path,
                const posix_spawn_file_actions_t *actions,
                const posix_spawnattr_t *attributes,
                char *const arguments[], char *const environment[]) {
    (void)process;
    (void)path;
    (void)actions;
    (void)attributes;
    (void)arguments;
    (void)environment;
    block_effect("child process spawned");
}

int posix_spawnp(pid_t *process, const char *file,
                 const posix_spawn_file_actions_t *actions,
                 const posix_spawnattr_t *attributes,
                 char *const arguments[], char *const environment[]) {
    (void)process;
    (void)file;
    (void)actions;
    (void)attributes;
    (void)arguments;
    (void)environment;
    block_effect("child process spawned");
}
