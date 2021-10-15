#include "network.h"

int
main()
{
    auto ts = network::tcp::connect("127.0.0.1:8080");
    return 0;
}
