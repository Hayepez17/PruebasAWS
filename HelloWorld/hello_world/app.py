import json


def lambda_handler(event, context):
    first_name = event['first_name']
    last_name = event['last_name']
    message = event['mesagge']

    return {
        "statusCode": 200,
        "body": json.dumps({
            "message": f"{mesagge} {first_name} {last_name}",
            # "location": ip.text.replace("\n", "")
        }),
    }