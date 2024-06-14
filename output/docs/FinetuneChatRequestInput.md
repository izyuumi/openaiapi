# FinetuneChatRequestInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::FinetuneChatRequestInputMessagesInner>**](FinetuneChatRequestInput_messages_inner.md)> |  | [optional]
**tools** | Option<[**Vec<models::ChatCompletionTool>**](ChatCompletionTool.md)> | A list of tools the model may generate JSON inputs for. | [optional]
**parallel_tool_calls** | Option<**bool**> | Whether to enable [parallel function calling](/docs/guides/function-calling/parallel-function-calling) during tool use. | [optional][default to true]
**functions** | Option<[**Vec<models::ChatCompletionFunctions>**](ChatCompletionFunctions.md)> | A list of functions the model may generate JSON inputs for. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


