#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "vendor/catch.hpp"

#include "libtrader.h"

bool test_list_node()
{
	struct LinkedListNode *prev = NULL, *node = NULL, *next = NULL;

	/* test node with no prev and next */
	node = createLinkedListNode(NULL, NULL, NULL);
	assert(node);
	free(node);

	/* test node with prev and next */
	prev = createLinkedListNode(NULL, node, NULL);
	node = createLinkedListNode(NULL, next, prev);
	next = createLinkedListNode(NULL, NULL, node);

	assert((int *)(node->prev) == (int *)prev);
	assert((int *)(node->next) == (int *)next);
	assert((int *)(prev->next) == (int *)node);
	assert((int *)(next->prev) == (int *)node);

	free(node);
	free(prev);
	free(next);

	/* test node with data */
	int *data = (int *)malloc(sizeof(int));
	memset(data, 42, sizeof(int));
	node = createLinkedListNode(&data, NULL, NULL);
	assert(node);
	assert(node->data);
	assert(node->data == &data);
	free(node);
	assert(data);

	return true;
}

int main()
{
	if (!test_list_node())
		return false;
	return 0;
}
