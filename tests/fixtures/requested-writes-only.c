#define _GNU_SOURCE

#include <dlfcn.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

static bool beneath(const char *path, const char *directory) {
    if (path == NULL || directory == NULL) return false;
    size_t length = strlen(directory);
    return strcmp(path, directory) == 0 ||
           (strncmp(path, directory, length) == 0 && path[length] == '/');
}

static bool allowed(const char *path) {
    const char *junit = getenv("ITE_ALLOWED_JUNIT");
    const char *evidence = getenv("ITE_ALLOWED_EVIDENCE_DIR");
    return (path != NULL && junit != NULL && strcmp(path, junit) == 0) ||
           beneath(path, evidence);
}

static void reject(const char *operation, const char *path) {
    const char prefix[] = "blocked unexpected filesystem write: ";
    (void)operation;
    (void)syscall(SYS_write, STDERR_FILENO, prefix, sizeof(prefix) - 1);
    if (path != NULL) (void)syscall(SYS_write, STDERR_FILENO, path, strlen(path));
    (void)syscall(SYS_write, STDERR_FILENO, "\n", 1);
    _exit(190);
}

static bool writes(int flags) {
    return (flags & O_ACCMODE) != O_RDONLY || (flags & (O_CREAT | O_TRUNC | O_APPEND)) != 0;
}

int open(const char *path, int flags, ...) {
    static int (*real_open)(const char *, int, ...) = NULL;
    if (real_open == NULL) real_open = dlsym(RTLD_NEXT, "open");
    mode_t mode = 0;
    if ((flags & O_CREAT) != 0) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    if (writes(flags) && !allowed(path)) reject("open", path);
    return (flags & O_CREAT) != 0 ? real_open(path, flags, mode) : real_open(path, flags);
}

int open64(const char *path, int flags, ...) {
    static int (*real_open64)(const char *, int, ...) = NULL;
    if (real_open64 == NULL) real_open64 = dlsym(RTLD_NEXT, "open64");
    mode_t mode = 0;
    if ((flags & O_CREAT) != 0) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    if (writes(flags) && !allowed(path)) reject("open64", path);
    return (flags & O_CREAT) != 0 ? real_open64(path, flags, mode) : real_open64(path, flags);
}

int openat(int directory, const char *path, int flags, ...) {
    static int (*real_openat)(int, const char *, int, ...) = NULL;
    if (real_openat == NULL) real_openat = dlsym(RTLD_NEXT, "openat");
    mode_t mode = 0;
    if ((flags & O_CREAT) != 0) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    if (writes(flags) && !allowed(path)) reject("openat", path);
    return (flags & O_CREAT) != 0 ? real_openat(directory, path, flags, mode) : real_openat(directory, path, flags);
}

int openat64(int directory, const char *path, int flags, ...) {
    static int (*real_openat64)(int, const char *, int, ...) = NULL;
    if (real_openat64 == NULL) real_openat64 = dlsym(RTLD_NEXT, "openat64");
    mode_t mode = 0;
    if ((flags & O_CREAT) != 0) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    if (writes(flags) && !allowed(path)) reject("openat64", path);
    return (flags & O_CREAT) != 0 ? real_openat64(directory, path, flags, mode) : real_openat64(directory, path, flags);
}

int mkdir(const char *path, mode_t mode) {
    static int (*real_mkdir)(const char *, mode_t) = NULL;
    if (real_mkdir == NULL) real_mkdir = dlsym(RTLD_NEXT, "mkdir");
    if (!allowed(path)) reject("mkdir", path);
    return real_mkdir(path, mode);
}

int mkdirat(int directory, const char *path, mode_t mode) {
    static int (*real_mkdirat)(int, const char *, mode_t) = NULL;
    if (real_mkdirat == NULL) real_mkdirat = dlsym(RTLD_NEXT, "mkdirat");
    if (!allowed(path)) reject("mkdirat", path);
    return real_mkdirat(directory, path, mode);
}
