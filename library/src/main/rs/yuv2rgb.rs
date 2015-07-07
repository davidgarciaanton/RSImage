/*
   Copyright 2012 Cesar Aguilar

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

#pragma version(1)
#pragma rs java_package_name(com.caguilar.android.filters.scripts)

int imageWidth;
int imageHeight;
rs_allocation yuvData;

void root(const uchar4 *v_in, uchar4 *v_out,const void *userData, uint32_t x, uint32_t y) {
    float r,g,b,Y,U,V;
    int index = y*imageWidth + x;
    int size = imageWidth*imageHeight;
    int otherIndex = ((y/2)*(imageWidth/2) + (x/2))*2 + size;


    uchar *temp = (uchar *)rsGetElementAt(yuvData,index);
    Y = *temp/255.0f;
    temp = (uchar *)rsGetElementAt(yuvData,otherIndex+1);
    U = *temp/255.0f;
    temp = (uchar *)rsGetElementAt(yuvData,otherIndex);
    V = *temp/255.0f;

    U -= 0.5f;
    V -= 0.5f;
    Y=1.1643*(Y-0.0625);
    r=Y+1.5958*V;
    g=Y-0.39173*U-0.81290*V;
    b=Y+2.017*U;

    float3 color = {r,g,b};
    color = clamp(color,0.0f,1.0f);
    *v_out = rsPackColorTo8888(color.r,color.g,color.b,1.0f);
}

void convert(rs_script script,rs_allocation inAllocation,rs_allocation outAllocation){
    rsForEach(script, inAllocation, outAllocation, 0, 0);
}