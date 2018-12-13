/*
    Basic and documented code that print messages
    int the tracer and blink the leds in the chip

    More info: https://ai-thinker-open.github.io/GPRS_C_SDK_DOC/en/c-sdk/first-code.html
*/

#include "stdbool.h"
#include "stdint.h"

#include <api_gps.h>

#include "api_os.h"
#include "api_debug.h"
#include "api_event.h"
#include "api_hal_gpio.h"
#include "gps_parse.h"
#include "gps.h"


#define MAIN_TASK_STACK_SIZE    (2048 * 2)
#define MAIN_TASK_PRIORITY      0
#define MAIN_TASK_NAME          "Main Test Task"

#define SECOND_TASK_STACK_SIZE    (2048 * 2)
#define SECOND_TASK_PRIORITY      1
#define SECOND_TASK_NAME          "Second Test Task"

static HANDLE mainTaskHandle = NULL;
static HANDLE secondTaskHandle = NULL;

/*void blinkD(UINT32 t, int nBlink){
    UINT32 counter = 0;
    UINT32 nDelay = t/nBlink;
    int ledVal = 0;

    while (counter <= t){

    }

}*/


void EventDispatch(API_Event_t* pEvent){
    switch(pEvent->id){
        case API_EVENT_ID_GPS_UART_RECEIVED:
            Trace(1,"#LOG: received GPS data,length:%d, data:%s",pEvent->param1,pEvent->pParam1);
            GPS_Update(pEvent->pParam1,pEvent->param1);
            break;
        default:
            break;
    }
}

// Secondary task where I'm going to execute everything.
void SecondTask(void *pData){

    GPS_Info_t* gpsInfo = Gps_GetInfo();
    uint8_t buffer[300];

    // GPIO configuration
    GPIO_config_t gpioLedBlue = {
        .mode         = GPIO_MODE_OUTPUT,
        .pin          = GPIO_PIN27,
        .defaultLevel = GPIO_LEVEL_LOW
    };

    GPIO_Init(gpioLedBlue); // Initialize GPIO

    GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_HIGH);
    OS_Sleep(250);
    GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_LOW);
    OS_Sleep(250);
    GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_HIGH);
    OS_Sleep(250);
    GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_LOW);
    OS_Sleep(2000);

    Trace(1, "#LOG: init gps...");

    if(!GPS_GetVersion(buffer,150))
        Trace(1,"#LOG: get gps firmware version fail");
    else
        Trace(1,"#LOG: gps firmware version:%s",buffer);

    //open GPS hardware(UART2 open either)
    GPS_Init();
    GPS_Open(NULL);

    //wait for gps start up, or gps will not response command
    while(gpsInfo->rmc.latitude.value == 0){
        GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_HIGH);
        OS_Sleep(500);
        GPIO_SetLevel(gpioLedBlue, GPIO_LEVEL_LOW);
        OS_Sleep(500);

        Trace(1, "#LOG: waiting...");
    }
}

// Main task
void MainTask(void *pData){
    API_Event_t* event=NULL;

    Trace(1, "Test #00002");

    // Create another task (secondary task).
    secondTaskHandle = OS_CreateTask(SecondTask,
        NULL, NULL, SECOND_TASK_STACK_SIZE, SECOND_TASK_PRIORITY, 0, 0, SECOND_TASK_NAME);

    while(1){
        // Wait for events from the lower system and is handle in EventDispatch
        if(OS_WaitEvent(mainTaskHandle, (void**)&event, OS_TIME_OUT_WAIT_FOREVER)){
            EventDispatch(event);
            OS_Free(event->pParam1);
            OS_Free(event->pParam2);
            OS_Free(event);
        }
    }
}

// Main entrance
void gpsTest_Main(void){
    // Create the main task. It will return the pointer os that task.
    mainTaskHandle = OS_CreateTask(MainTask,
        NULL, NULL, MAIN_TASK_STACK_SIZE, MAIN_TASK_PRIORITY, 0, 0, MAIN_TASK_NAME);
    OS_SetUserMainHandle(&mainTaskHandle); // Set the main handle.
}