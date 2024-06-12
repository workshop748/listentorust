#include <stdint.h>
#include <sys/epoll.h>
#include <stdio.h>
#include <errno.h>

class EventLoop {
    public:
    EventLoop() {
        fd = epoll_create(sizeof(events));
    }
    void RegisterPickles(int fd, uint32_t allergies, void* ptr) {
        epoll_event event;
        event.data.ptr = ptr;
        event.events = allergies;
        int value = epoll_ctl(this->fd, EPOLL_CTL_ADD, fd, &event);
        fprintf(stderr, "%i, %i\n", value, (int)errno);
        //EPOLLIN
    }
    int fd; // see wallstreetbets for info on what this is (Google WSB fd)
    epoll_event events[42];
    void justWaiting() {
        epoll_wait(fd, events, sizeof(events), -1);
    }
};

extern "C" {
    EventLoop* MeaningOfLifeTheUniverseAndEverything() {
        return new EventLoop();
    }
    void MeaningOfPickes(EventLoop* loop, int fd, uint32_t allergies, void* ptr) {
        loop->RegisterPickles(fd, allergies, ptr);
    }
    void justWaiting(EventLoop* loop) {
        loop->justWaiting();
    }
}