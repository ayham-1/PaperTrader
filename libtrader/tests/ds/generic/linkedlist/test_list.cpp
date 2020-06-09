#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#define CATCH_CONFIG_MAIN
#include "vendor/catch.hpp"

#include "libtrader.h"

TEST_CASE("linkedListNode testing", "[node]")
{
	struct LinkedListNode *prev = NULL, *node = NULL, *next = NULL;

	/* test node with no prev and next */
	node = createLinkedListNode(NULL, NULL, NULL);
	REQUIRE(node);
	free(node);

	/* test node with prev and next */
	next = createLinkedListNode(NULL, NULL, NULL);
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, prev, next);

	REQUIRE((int *)(node->prev) == (int *)prev);
	REQUIRE((int *)(node->next) == (int *)next);

	free(node);
	free(prev);
	free(next);

	/* test node with data */
	int *data = (int *)malloc(sizeof(int));
	memset(data, 42, sizeof(int));
	node = createLinkedListNode(&data, NULL, NULL);
	REQUIRE(node);
	REQUIRE(node->data);
	REQUIRE(node->data == &data);
	free(node);
	REQUIRE(data);
}
